use std::io::Write;

use backend::Backend;
use types::ProgramToken;

pub mod code_stream;
mod module_builder;
use self::module_builder::WasmModule;

pub struct WasmBackend;

impl Backend for WasmBackend {
  fn compile_to_stream(&self, tokens: Vec<ProgramToken>, stream: &mut impl Write) {
    WasmModule::write_to_stream(stream, tokens).unwrap();
  }
}
