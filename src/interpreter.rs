use std;
use std::io::Read;

use types::{ProgramToken, State};

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

pub fn run_program(program: &[ProgramToken], state: &mut State, io: &mut impl BfIo) {
  let mut instruction_pointer = 0;

  use types::MemoryOp::*;
  use types::ProgramToken::*;

  while let Some(op) = program.get(instruction_pointer) {
    instruction_pointer += 1;
    match op {
      ChangeAddr(by) => {
        state.pointer = ((state.pointer as isize) + by) as usize;
      }
      Loop(body) => {
        while state.memory[state.pointer as usize] != 0 {
          run_program(body, state, io);
        }
      }
      Offset(offset, op) => {
        let address = (state.pointer as isize)
          .checked_add(*offset as isize)
          .expect("Pointer shouldn't over- or underflow.") as usize;

        match op {
          ChangeValue(value) => {
            state.memory[address] =
              ((state.memory[address] as isize).wrapping_add(*value as isize)) as u8;
          }
          SetValue(value) => {
            state.memory[address] = *value as u8;
          }
          Print => {
            io.print(state.memory[address]);
          }
        }
      }
    }
  }
}
