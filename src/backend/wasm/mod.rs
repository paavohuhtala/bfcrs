use std::fmt::Write;

use backend::Backend;
use ProgramToken;

pub struct WasmBackend;

impl Backend for WasmBackend {
  type CompilationResult = String;

  fn compile(tokens: impl Iterator<Item = ProgramToken>) -> String {
    let mut result = String::new();

    write!(&mut result, "(module)").unwrap();

    result
  }
}
