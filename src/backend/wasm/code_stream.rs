use std::error::Error;
use std::io::Write;

use byteorder::WriteBytesExt;
use leb128;

use backend::wasm::module_builder::WasmType;

pub enum Instruction {
  GetLocal(LocalHandle),
  SetLocal(LocalHandle),
  Load8Signed(u32),
  Load8Unsigned(u32),
  Store8(u32),
  PushI32(i32),
  AddI32,
  Call(u32),
  BranchIf(u32),
  EqualsZeroI32,
  Loop,
  Block,
  End,
  Drop,
  Return,
}

pub trait CodeStreamExt {
  fn write_leb_u8(&mut self, x: u8);
  fn write_leb_i8(&mut self, x: i8);
  fn write_leb_u32(&mut self, x: u32);
  fn write_leb_i32(&mut self, x: i32);
  fn write_local(&mut self, handle: LocalHandle);
  fn write_str(&mut self, x: &str);
}

impl<T: Write> CodeStreamExt for T {
  fn write_leb_u8(&mut self, x: u8) {
    leb128::write::unsigned(self, x as u64).unwrap();
  }

  fn write_leb_i8(&mut self, x: i8) {
    leb128::write::signed(self, x as i64).unwrap();
  }

  fn write_leb_u32(&mut self, x: u32) {
    leb128::write::unsigned(self, x as u64).unwrap();
  }

  fn write_leb_i32(&mut self, x: i32) {
    leb128::write::signed(self, x as i64).unwrap();
  }

  fn write_local(&mut self, handle: LocalHandle) {
    self.write_leb_u32(handle.0);
  }

  fn write_str(&mut self, x: &str) {
    self.write_leb_u32(x.len() as u32);
    self.write(x.as_bytes()).unwrap();
  }
}

#[derive(Copy, Clone)]
pub struct LocalHandle(pub u32);

pub struct CodeStreamWriter<'a, T: Write + 'a> {
  stream: &'a mut T,
  locals: Vec<WasmType>,
}

impl<'a, T: Write + 'a> CodeStreamWriter<'a, T> {
  pub fn new(stream: &mut T) -> CodeStreamWriter<T> {
    CodeStreamWriter {
      stream,
      locals: Vec::new(),
    }
  }

  pub fn declare_local(&mut self, local_type: WasmType) -> LocalHandle {
    let handle = LocalHandle(self.locals.len() as u32);
    self.locals.push(local_type);
    handle
  }

  pub fn emit_print_string(&mut self, s: &str) -> Result<(), Box<Error>> {
    for b in s.bytes() {
      self.emit(Instruction::PushI32(b as i32))?;
      self.emit(Instruction::Call(0))?;
    }
    Ok(())
  }

  pub fn emit(&mut self, op: Instruction) -> Result<(), Box<Error>> {
    use self::Instruction::*;

    match op {
      GetLocal(handle) => {
        self.stream.write_u8(0x20)?;
        self.stream.write_local(handle);
      }
      SetLocal(handle) => {
        self.stream.write_u8(0x21)?;
        self.stream.write_local(handle);
      }
      PushI32(value) => {
        self.stream.write_u8(0x41)?;
        self.stream.write_leb_i32(value);
      }
      Load8Signed(offset) => {
        self.stream.write_u8(0x2C)?;
        self.stream.write_leb_u32(0);
        self.stream.write_leb_u32(offset);
      }
      Load8Unsigned(offset) => {
        self.stream.write_u8(0x2D)?;
        self.stream.write_leb_u32(0);
        self.stream.write_leb_u32(offset);
      }
      Store8(offset) => {
        self.stream.write_u8(0x3A)?;
        self.stream.write_leb_u32(0);
        self.stream.write_leb_u32(offset);
      }
      AddI32 => {
        self.stream.write_u8(0x6A)?;
      }
      Call(function) => {
        self.stream.write_u8(0x10)?;
        self.stream.write_leb_u32(function);
      }
      BranchIf(depth) => {
        self.stream.write_u8(0x0D)?;
        self.stream.write_leb_u32(depth);
      }
      EqualsZeroI32 => {
        self.stream.write_u8(0x45)?;
      }
      Loop => {
        self.stream.write_u8(0x03)?;
        // Loops can return values, but 0x40 indicates this one doesn't.
        self.stream.write_u8(0x40)?;
      }
      Block => {
        self.stream.write_u8(0x02)?;
        self.stream.write_u8(0x40)?;
      }
      End => {
        self.stream.write_u8(0x0B)?;
      }
      Drop => {
        self.stream.write_u8(0x1A)?;
      }
      Return => {
        self.stream.write_u8(0x0F)?;
      }
    }

    Ok(())
  }
}
