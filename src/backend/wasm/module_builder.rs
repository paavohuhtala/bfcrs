use std::error::Error;
use std::io::Write;

use byteorder::{LittleEndian, WriteBytesExt};

use backend::wasm::code_stream::LocalHandle;
use backend::wasm::code_stream::{CodeStreamExt, CodeStreamWriter, Instruction};

use types::MemoryOp::*;
use types::ProgramToken;
use types::ProgramToken::*;

struct Section {
  id: u8,
  data: Vec<u8>,
}

pub struct ModuleBuilder {
  sections: Vec<Section>,
}

impl ModuleBuilder {
  fn new() -> ModuleBuilder {
    ModuleBuilder {
      sections: Vec::new(),
    }
  }

  pub fn add_section(&mut self, id: u8, entries: &[Box<Fn(&mut dyn Write) -> ()>]) {
    let mut data = Vec::new();
    data.write_leb_u32(entries.len() as u32);

    for entry_fn in entries {
      entry_fn(&mut data);
    }

    self.sections.push(Section { id, data });
  }

  pub fn write_to_stream(mut self, mut stream: &mut dyn Write) -> Result<(), Box<Error>> {
    let sorted = self.sections.as_mut_slice();
    sorted.sort_unstable_by_key(|x| x.id);

    // Header:
    // Magic
    stream.write_u32::<LittleEndian>(0x6d736100)?;
    // Version
    stream.write_u32::<LittleEndian>(1)?;

    for section in sorted {
      stream.write_u8(section.id)?;
      stream.write_leb_u32(section.data.len() as u32);
      stream.write(&section.data)?;
    }

    Ok(())
  }
}

pub struct WasmModule;

fn emit_token<T: Write>(
  writer: &mut CodeStreamWriter<T>,
  pointer: LocalHandle,
  token: &ProgramToken,
) -> Result<(), Box<Error>> {
  use self::Instruction::*;

  match token {
    ChangeAddr(by) => {
      writer.emit(GetLocal(pointer))?;
      writer.emit(PushI32(*by as i32))?;
      writer.emit(AddI32)?;
      writer.emit(SetLocal(pointer))?;
    }
    Offset(0, ChangeValue(value)) => {
      writer.emit(GetLocal(pointer))?;
      writer.emit(GetLocal(pointer))?;
      writer.emit(Load8Unsigned(0))?;
      writer.emit(PushI32(*value as i32))?;
      writer.emit(AddI32)?;
      writer.emit(Store8(0))?;
    }
    // If the offset is positive, we can take advantage of indexed load/store
    Offset(addr_offset, ChangeValue(value)) if *addr_offset > 0 => {
      // Push the pointer in preparation for store
      writer.emit(GetLocal(pointer))?;

      // Compute value
      writer.emit(GetLocal(pointer))?;
      writer.emit(Load8Unsigned(*addr_offset as u32))?;
      writer.emit(PushI32(*value as i32))?;
      writer.emit(AddI32)?;

      // Store result
      writer.emit(Store8(*addr_offset as u32))?;
    }
    Offset(addr_offset, ChangeValue(value)) => {
      // Compute address
      writer.emit(GetLocal(pointer))?;
      writer.emit(PushI32(*addr_offset as i32))?;
      writer.emit(AddI32)?;

      // Why doesn't WASM have a DUP instruction?
      writer.emit(GetLocal(pointer))?;
      writer.emit(PushI32(*addr_offset as i32))?;
      writer.emit(AddI32)?;

      // Compute value
      writer.emit(Load8Unsigned(0))?;
      writer.emit(PushI32(*value as i32))?;
      writer.emit(AddI32)?;

      // Store result
      writer.emit(Store8(0))?;
    }
    Offset(0, Print) => {
      writer.emit(GetLocal(pointer))?;
      writer.emit(Load8Unsigned(0))?;
      writer.emit(Call(0))?;
    }
    Offset(addr_offset, Print) if *addr_offset > 0 => {
      writer.emit(GetLocal(pointer))?;
      writer.emit(Load8Unsigned(*addr_offset as u32))?;
      writer.emit(Call(0))?;
    }
    Offset(addr_offset, Print) => {
      writer.emit(GetLocal(pointer))?;
      writer.emit(PushI32(*addr_offset as i32))?;
      writer.emit(AddI32)?;

      writer.emit(Load8Unsigned(0))?;
      writer.emit(Call(0))?;
    }
    Offset(0, SetValue(value)) => {
      writer.emit(GetLocal(pointer))?;
      writer.emit(PushI32(*value as i32))?;
      writer.emit(Store8(0))?;
    }
    Offset(addr_offset, SetValue(value)) if *addr_offset > 0 => {
      writer.emit(GetLocal(pointer))?;
      writer.emit(PushI32(*value as i32))?;
      writer.emit(Store8(*addr_offset as u32))?;
    }
    Offset(addr_offset, SetValue(value)) => {
      writer.emit(GetLocal(pointer))?;
      writer.emit(PushI32(*addr_offset as i32))?;
      writer.emit(AddI32)?;
      writer.emit(PushI32(*value as i32))?;
      writer.emit(Store8(0))?;
    }
    ProgramToken::Loop(body) => {
      // This is essentially compiled into the following pseudocode:
      // if memory[pointer] != 0 {
      //   do {
      //     * stuff*
      //   } while memory[pointer != 0]
      // }

      writer.emit(Block)?;

      writer.emit(GetLocal(pointer))?;
      writer.emit(Load8Unsigned(0))?;
      writer.emit(EqualsZeroI32)?;
      writer.emit(BranchIf(0))?;

      writer.emit(Loop)?;

      for token in body {
        emit_token(writer, pointer, token)?;
      }

      writer.emit(GetLocal(pointer))?;
      writer.emit(Load8Unsigned(0))?;
      writer.emit(BranchIf(0))?;

      writer.emit(End)?;
      writer.emit(End)?;
    }
  }
  Ok(())
}

fn add_type_section(builder: &mut ModuleBuilder) {
  builder.add_section(
    1,
    &[
      // print
      Box::new(|mut writer| {
        writer.write_u8(0x60).unwrap();
        // One integer param
        writer.write_leb_u32(1);
        writer.write_u8(0x7F).unwrap();
        // Doesn't read anything
        writer.write_leb_u32(0);
      }),
      // read
      Box::new(|mut writer| {
        writer.write_u8(0x60).unwrap();
        // No params
        writer.write_leb_u32(0);
        // Returns an integer
        writer.write_leb_u32(1);
        writer.write_u8(0x7F).unwrap();
      }),
      // Main
      Box::new(|mut writer| {
        writer.write_u8(0x60).unwrap();
        // No params
        writer.write_leb_u32(0);
        // Returns the instruction pointer
        writer.write_leb_u32(1);
        writer.write_u8(0x7F).unwrap();
      }),
    ],
  )
}

fn add_import_section(builder: &mut ModuleBuilder) {
  builder.add_section(
    2,
    &[
      Box::new(|mut writer| {
        writer.write_str("bfcrs");
        writer.write_str("print");
        writer.write_u8(0).unwrap();
        writer.write_leb_u32(0);
      }),
      Box::new(|mut writer| {
        writer.write_str("bfcrs");
        writer.write_str("read");
        writer.write_u8(0).unwrap();
        writer.write_leb_u32(1);
      }),
    ],
  );
}

fn add_memory_section(builder: &mut ModuleBuilder, page_count: u32) {
  builder.add_section(
    5,
    &[Box::new(move |mut writer| {
      // resizable_limits.flags
      writer.write_u8(0).unwrap();
      // resizable_limits.initial
      writer.write_leb_u32(page_count);
    })],
  );
}

fn add_function_section(builder: &mut ModuleBuilder) {
  builder.add_section(
    3,
    &[Box::new(|writer| {
      writer.write_u8(2).unwrap();
    })],
  );
}

fn add_export_section(builder: &mut ModuleBuilder) {
  builder.add_section(
    7,
    &[
      Box::new(|mut writer| {
        writer.write_str("main");
        writer.write_u8(0).unwrap();
        writer.write_u8(2).unwrap();
      }),
      Box::new(|mut writer| {
        writer.write_str("memory");
        writer.write_u8(2).unwrap();
        writer.write_u8(0).unwrap();
      }),
    ],
  );
}

fn add_code_section(builder: &mut ModuleBuilder, tokens: Vec<ProgramToken>) {
  builder.add_section(
    10,
    &[Box::new(move |writer| {
      let mut code: Vec<u8> = Vec::new();

      {
        let mut writer = CodeStreamWriter::new(&mut code);
        let pointer = writer.declare_local(WasmType::I32);

        for token in &tokens {
          emit_token(&mut writer, pointer, token).unwrap();
        }

        writer.emit(Instruction::GetLocal(pointer)).unwrap();
        writer.emit(Instruction::Return).unwrap();

        writer.emit(Instruction::End).unwrap();
      }

      let mut code_body: Vec<u8> = Vec::new();
      // Number of locals
      code_body.write_leb_u32(1);
      // Number of locals of this type
      code_body.write_leb_u32(1);
      // Type of the local
      code_body.write_u8(0x7F).unwrap();

      code_body.write(&code).unwrap();

      let mut code_entry: Vec<u8> = Vec::new();
      code_entry.write_leb_u32(code_body.len() as u32);
      code_entry.write(&code_body).unwrap();

      writer.write(&code_entry).unwrap();
    })],
  );
}

impl WasmModule {
  pub fn write_to_stream(
    stream: &mut dyn Write,
    tokens: &[ProgramToken],
  ) -> Result<(), Box<Error>> {
    let mut builder = ModuleBuilder::new();

    add_type_section(&mut builder);
    add_import_section(&mut builder);
    add_function_section(&mut builder);
    add_memory_section(&mut builder, 1);
    add_export_section(&mut builder);
    add_code_section(&mut builder, tokens.to_vec());

    builder.write_to_stream(stream)?;

    Ok(())
  }
}

#[derive(Copy, Clone)]
pub enum WasmType {
  I32,
}
