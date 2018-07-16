use std::io::Write;
use types::ProgramToken;

pub mod c;
pub mod wasm;

pub trait Backend {
  fn extension(&self) -> &'static str;
  fn compile_to_stream(&self, tokens: &[ProgramToken], stream: &mut dyn Write);
}

impl Backend {
  pub fn from_name(name: &str) -> Option<Box<Backend>> {
    match name {
      "c" => Some(Box::new(self::c::CBackend)),
      "wasm" => Some(Box::new(self::wasm::WasmBackend)),
      _ => None,
    }
  }
}
