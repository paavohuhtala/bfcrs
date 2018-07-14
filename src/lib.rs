#![feature(iterator_find_map)]
#![feature(iterator_flatten)]
#![feature(slice_patterns)]

extern crate byteorder;
extern crate leb128;

pub mod backend;
pub mod interpreter;
pub mod optimizer;
pub mod parser;
pub mod types;

use types::ProgramToken;

pub fn compile_program(source: &str) -> Vec<u8> {
  let tokens = parser::parse_program(source);
  let program = optimizer::convert_tokens(&tokens);
  compile_tokens(&program, true)
}

pub fn compile_tokens(tokens: &[ProgramToken], optimize: bool) -> Vec<u8> {
  let optimized = if optimize {
    optimizer::optimize(tokens)
  } else {
    tokens.to_vec()
  };

  let mut code = Vec::new();

  let backend = backend::wasm::WasmBackend;
  use backend::Backend;
  backend.compile_to_stream(&optimized, &mut code);
  code
}
