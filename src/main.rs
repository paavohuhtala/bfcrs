extern crate bfcrs;

use std::fs::{create_dir_all, read_to_string, OpenOptions};
use std::path::Path;

use bfcrs::backend::wasm::WasmBackend;
use bfcrs::backend::Backend;
use bfcrs::interpreter::ConsoleIo;
use bfcrs::interpreter::{run_program, State};
use bfcrs::optimizer::optimize;
use bfcrs::parser::parse_program;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let src_file = args
    .get(1)
    .map(|arg| arg.to_string())
    .unwrap_or_else(|| "./bf/hello.bf".to_string());
  let src = read_to_string(src_file).expect("Source file should exist.");

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

  run_program(
    &optimized_program.clone(),
    &mut State::new(),
    &mut ConsoleIo,
  );

  let wasm_backend = WasmBackend;
  wasm_backend.compile_to_stream(optimized_program, &mut output_file);
}
