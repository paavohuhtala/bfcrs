use std;
use std::io::Read;

use utils::vec_utils::VecUtils;
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

pub fn run_program(program: Vec<ProgramToken>, io: &mut impl BfIo) {
  let mut memory = vec![0u8; std::u16::MAX as usize];
  let mut pointer: u16 = 0;
  let mut instruction_pointer = 0;

  use ProgramToken::*;

  while let Some(op) = program.get(instruction_pointer) {
    match op {
      ChangeAddr(by) => pointer = by.wrapping_add(pointer as i32) as u16,
      ChangeValue(by) => {
        memory[pointer as usize] = by.wrapping_add(memory[pointer as usize] as i32) as u8
      }
      LoopStart => {
        if memory[pointer as usize] == 0 {
          if let Some(offset) = program.find_index(&LoopEnd, instruction_pointer) {
            instruction_pointer = offset + 1;
            continue;
          } else {
            panic!("Unmatched '['");
          }
        }
      }
      LoopEnd => {
        if memory[pointer as usize] != 0 {
          if let Some(offset) = program.find_index_backwards(&LoopStart, instruction_pointer) {
            instruction_pointer = offset + 1;
            continue;
          } else {
            panic!("Unmatched ']'");
          }
        }
      }
      Print => io.print(memory[pointer as usize]),
    }
    instruction_pointer += 1;
  }
}
