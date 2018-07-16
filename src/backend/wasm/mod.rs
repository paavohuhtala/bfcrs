use std::io::Write;

use backend::Backend;
use types::ProgramToken;

pub mod code_stream;
mod module_builder;
use self::module_builder::WasmModule;

pub struct WasmBackend;

impl Backend for WasmBackend {
  fn extension(&self) -> &'static str {
    ".wasm"
  }

  fn compile_to_stream(&self, tokens: &[ProgramToken], stream: &mut dyn Write) {
    WasmModule::write_to_stream(stream, tokens).unwrap();
  }
}
