use std::io::Write;
use ProgramToken;

pub mod wasm;

pub trait Backend {
  fn compile_to_stream(&self, tokens: Vec<ProgramToken>, stream: &mut impl Write);
}
