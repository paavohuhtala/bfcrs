use std;
use std::io::Read;

use ProgramToken;

pub trait BfIo {
  fn print(&mut self, ch: u8);
  fn read(&mut self) -> u8;
}

pub struct ConsoleIo;

impl BfIo for ConsoleIo {
  fn print(&mut self, ch: u8) {
    print!("{}", ch as char);
  }

  fn read(&mut self) -> u8 {
    let mut buffer = [0u8];
    std::io::stdin().read_exact(&mut buffer).unwrap();
    buffer[0]
  }
}

pub struct State {
  memory: Vec<u8>,
  pointer: usize,
}

impl State {
  pub fn new() -> State {
    State {
      memory: vec![0u8; std::u16::MAX as usize],
      pointer: 0,
    }
  }
}

pub fn run_program(program: &[ProgramToken], state: &mut State, io: &mut impl BfIo) {
  let mut instruction_pointer = 0;

  use ProgramToken::*;

  while let Some(op) = program.get(instruction_pointer) {
    match op {
      ChangeAddr(by) => {
        state.pointer = ((state.pointer as isize) + by) as usize;
        instruction_pointer += 1;
      }
      ChangeValue(by) => {
        state.memory[state.pointer] =
          ((state.memory[state.pointer] as isize).checked_add(*by as isize))
            .expect("Pointer shouldn't over- or underflow") as u8;
        instruction_pointer += 1;
      }
      ChangeOffset { addr_offset, value } => {
        let address = (state.pointer as isize)
          .checked_add(*addr_offset as isize)
          .expect("Pointer shouldn't over- or underflow.") as usize;

        state.memory[address] =
          ((state.memory[address] as isize).wrapping_add(*value as isize)) as u8;
        instruction_pointer += 1;
      }
      Zero => {
        state.memory[state.pointer as usize] = 0;
        instruction_pointer += 1;
      }
      Loop(body) => {
        instruction_pointer += 1;
        while state.memory[state.pointer as usize] != 0 {
          run_program(body, state, io);
        }
      }
      Print => {
        io.print(state.memory[state.pointer as usize]);
        instruction_pointer += 1;
      }
    }
  }
}
