#![feature(iterator_find_map)]
#![feature(iterator_flatten)]

mod utils;

mod parser;
use parser::parse_program;

mod optimizer;
use optimizer::optimize;

mod interpreter;
use interpreter::run_program;

mod backend;

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
  let hello_world = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
  let parsed_program = parse_program(hello_world);
  let optimized_program = optimize(parsed_program);
  println!("{:?}", optimized_program);
  run_program(optimized_program);
}
