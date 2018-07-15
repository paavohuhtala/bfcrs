use bfcrs::interpreter::{run_program, BfIo};
use bfcrs::optimizer::optimize_parsed;
use bfcrs::parser::parse_program;
use bfcrs::types::{ProgramToken, State};
use common::types::RunResult;

struct MockIo {
  output: String,
}

impl BfIo for MockIo {
  fn print(&mut self, ch: u8) {
    self.output.push(ch.into());
  }

  fn read(&mut self) -> u8 {
    0
  }
}

pub fn run_tokens_in_interpreter(program: &[ProgramToken]) -> RunResult {
  let mut state = State::new();
  let mut io = MockIo {
    output: String::new(),
  };

  run_program(program, &mut state, &mut io);

  RunResult {
    output: io.output,
    state,
  }
}

pub fn run_bf_in_interpreter(source: &str) -> RunResult {
  let program = optimize_parsed(&parse_program(source));
  run_tokens_in_interpreter(&program)
}
