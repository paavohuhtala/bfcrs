#![feature(iterator_find_map)]
#![feature(iterator_flatten)]

extern crate byteorder;
extern crate leb128;

mod utils;

mod parser;
use interpreter::ConsoleIo;
use parser::parse_program;

mod optimizer;
use optimizer::optimize;

mod interpreter;
use interpreter::run_program;

mod backend;
use backend::wasm::WasmBackend;
use backend::Backend;

use std::fs::{create_dir_all, read_to_string, OpenOptions};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum ParseToken {
  IncrAddr,
  DecrAddr,
  IncrValue,
  DecrValue,
  LoopStart,
  LoopEnd,
  Print,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProgramToken {
  ChangeValue(i32),
  ChangeAddr(i32),
  LoopStart,
  LoopEnd,
  Print,
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let src_file = args
    .get(1)
    .map(|arg| arg.to_string())
    .unwrap_or_else(|| "./bf/hello.bf".to_string());
  let src = read_to_string(src_file).expect("File source file should exist.");

  let parsed_program = parse_program(&src);
  let optimized_program = optimize(parsed_program);
  println!("{:?}", optimized_program);

  create_dir_all("./bin").unwrap();

  let output_path = Path::new("./bin").join("out.wasm");

  let mut output_file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(output_path)
    .unwrap();

  run_program(optimized_program.clone(), &mut ConsoleIo);

  let wasm_backend = WasmBackend;
  wasm_backend.compile_to_stream(optimized_program, &mut output_file);
}
