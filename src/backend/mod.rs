use ProgramToken;

pub mod wasm;

pub trait Backend {
  type CompilationResult;

  fn compile(tokens: impl Iterator<Item = ProgramToken>) -> Self::CompilationResult;
}
