use std::error::Error;
use std::io::Write;

use byteorder::{LittleEndian, WriteBytesExt};

use backend::wasm::code_stream::LocalHandle;
use backend::wasm::code_stream::{CodeStreamExt, CodeStreamWriter, Instruction};
use types::ProgramToken;

pub struct WasmModule;

impl WasmModule {
  fn write_header(stream: &mut impl Write) -> Result<(), Box<Error>> {
    // Magic
    stream.write_u32::<LittleEndian>(0x6d736100)?;
    // Version
    stream.write_u32::<LittleEndian>(1)?;

    Ok(())
  }

  fn write_memory_section(stream: &mut impl Write, memory_size: u32) -> Result<(), Box<Error>> {
    let mut memory_section: Vec<u8> = Vec::new();
    // memory.count
    memory_section.write_leb_u32(1);
    // resizable_limits.flags
    memory_section.write_u8(0)?;
    // resizable_limits.initial
    memory_section.write_leb_u32(memory_size);

    stream.write_u8(5)?;
    stream.write_leb_u32(memory_section.len() as u32);
    stream.write(&memory_section)?;

    Ok(())
  }

  fn write_types_and_imports(stream: &mut impl Write) -> Result<(), Box<Error>> {
    let mut type_section: Vec<u8> = Vec::new();
    // There will be 3 signatures:
    type_section.write_u8(3)?;

    // print
    type_section.write_u8(0x60)?;
    // One integer param
    type_section.write_leb_u32(1);
    type_section.write_u8(0x7F)?;
    // Doesn't read anything
    type_section.write_leb_u32(0);

    // read
    type_section.write_u8(0x60)?;
    // No params
    type_section.write_leb_u32(0);
    // Returns an integer
    type_section.write_leb_u32(1);
    type_section.write_u8(0x7F)?;

    // main
    type_section.write_u8(0x60)?;
    // No params
    type_section.write_leb_u32(0);
    // Returns the instruction pointer
    type_section.write_leb_u32(1);
    type_section.write_u8(0x7F)?;

    stream.write_u8(1)?;
    stream.write_leb_u32(type_section.len() as u32);
    stream.write(&type_section)?;

    let mut import_section: Vec<u8> = Vec::new();

    import_section.write_u8(2)?;

    import_section.write_str("bfcrs");
    import_section.write_str("print");
    import_section.write_u8(0)?;
    import_section.write_leb_u32(0);

    import_section.write_str("bfcrs");
    import_section.write_str("read");
    import_section.write_u8(0)?;
    import_section.write_leb_u32(1);

    stream.write_u8(2)?;
    stream.write_leb_u32(import_section.len() as u32);
    stream.write(&import_section)?;

    Ok(())
  }

  fn write_function_section(stream: &mut impl Write) -> Result<(), Box<Error>> {
    // Functions
    let mut function_section: Vec<u8> = Vec::new();
    function_section.write_leb_u32(1);
    function_section.write_leb_u32(2);

    stream.write_u8(3)?;
    stream.write_leb_u32(function_section.len() as u32);
    stream.write(&function_section)?;

    Ok(())
  }

  fn write_export_section(stream: &mut impl Write) -> Result<(), Box<Error>> {
    let mut export_section: Vec<u8> = Vec::new();
    export_section.write_u8(2)?;

    export_section.write_str("main");
    export_section.write_u8(0)?;
    export_section.write_u8(2)?;

    export_section.write_str("memory");
    export_section.write_u8(2)?;
    export_section.write_u8(0)?;

    // Export
    stream.write_u8(7)?;
    stream.write_leb_u32(export_section.len() as u32);
    stream.write(&export_section)?;

    Ok(())
  }

  fn write_code_section(
    stream: &mut impl Write,
    tokens: &[ProgramToken],
  ) -> Result<(), Box<Error>> {
    let mut code: Vec<u8> = Vec::new();
    {
      let mut writer = CodeStreamWriter::new(&mut code);
      let pointer = writer.declare_local(WasmType::I32);

      use self::Instruction::*;

      fn write_tokens<T: Write>(
        writer: &mut CodeStreamWriter<T>,
        tokens: &[ProgramToken],
        pointer: LocalHandle,
      ) -> Result<(), Box<Error>> {
        for token in tokens {
          match token {
            ProgramToken::ChangeValue(by) => {
              writer.emit(GetLocal(pointer))?;
              writer.emit(GetLocal(pointer))?;
              writer.emit(Load8Unsigned(0))?;
              writer.emit(PushI32(*by as i32))?;
              writer.emit(AddI32)?;
              writer.emit(Store8(0))?;
            }
            ProgramToken::ChangeAddr(by) => {
              writer.emit(GetLocal(pointer))?;
              writer.emit(PushI32(*by as i32))?;
              writer.emit(AddI32)?;
              writer.emit(SetLocal(pointer))?;
            }
            ProgramToken::ChangeOffset { addr_offset, value } => {
              // Compute address
              writer.emit(GetLocal(pointer))?;
              writer.emit(PushI32(*addr_offset as i32))?;
              writer.emit(AddI32)?;

              // Compute value
              writer.emit(GetLocal(pointer))?;
              writer.emit(Load8Unsigned(0))?;
              writer.emit(PushI32(*value as i32))?;
              writer.emit(AddI32)?;

              // Store result
              writer.emit(Store8(0))?;
            }
            ProgramToken::Print => {
              writer.emit(GetLocal(pointer))?;
              writer.emit(Load8Unsigned(0))?;
              writer.emit(Call(0))?;
            }
            ProgramToken::Zero => {
              writer.emit(GetLocal(pointer))?;
              writer.emit(PushI32(0))?;
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

              write_tokens(writer, &body, pointer)?;

              writer.emit(GetLocal(pointer))?;
              writer.emit(Load8Unsigned(0))?;
              writer.emit(BranchIf(0))?;

              writer.emit(End)?;
              writer.emit(End)?;
            }
          }
        }
        Ok(())
      }

      write_tokens(&mut writer, tokens, pointer)?;

      writer.emit(Instruction::GetLocal(pointer))?;
      writer.emit(Instruction::Return)?;

      writer.emit(Instruction::End)?;
    }

    let mut code_body: Vec<u8> = Vec::new();
    // Number of locals
    code_body.write_leb_u32(1);
    // Number of locals of this type
    code_body.write_leb_u32(1);
    // Type of the local
    code_body.write_u8(0x7F)?;

    code_body.write(&code)?;

    let mut code_entry: Vec<u8> = Vec::new();
    code_entry.write_leb_u32(code_body.len() as u32);
    code_entry.write(&code_body)?;

    let mut code_section: Vec<u8> = Vec::new();
    code_section.write_leb_u32(1);
    code_section.write(&code_entry)?;

    // Code
    stream.write_u8(10)?;
    stream.write_leb_u32(code_section.len() as u32);
    stream.write(&code_section)?;

    Ok(())
  }

  pub fn write_to_stream(
    stream: &mut impl Write,
    tokens: &[ProgramToken],
  ) -> Result<(), Box<Error>> {
    Self::write_header(stream)?;
    Self::write_types_and_imports(stream)?;
    Self::write_function_section(stream)?;
    Self::write_memory_section(stream, 1)?;
    Self::write_export_section(stream)?;
    Self::write_code_section(stream, &tokens)?;

    Ok(())
  }
}

#[derive(Copy, Clone)]
pub enum WasmType {
  I32,
}
