#![feature(iterator_find_map)]
#![feature(iterator_flatten)]

mod utils;
use utils::iter_utils::IterUtils;
use utils::vec_utils::VecUtils;

#[derive(Debug, PartialEq)]
enum ParseToken {
  IncrAddr,
  DecrAddr,
  IncrValue,
  DecrValue,
  LoopStart,
  LoopEnd,
  Print,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ProgramToken {
  ChangeValue(i32),
  ChangeAddr(i32),
  LoopStart,
  LoopEnd,
  Print,
}

fn parse_program(program: &str) -> Vec<ParseToken> {
  program
    .chars()
    .filter_map(|x| match x {
      '>' => Some(ParseToken::IncrAddr),
      '<' => Some(ParseToken::DecrAddr),
      '+' => Some(ParseToken::IncrValue),
      '-' => Some(ParseToken::DecrValue),
      '[' => Some(ParseToken::LoopStart),
      ']' => Some(ParseToken::LoopEnd),
      '.' => Some(ParseToken::Print),
      _ => None,
    })
    .collect()
}

fn try_fuse(a: &ProgramToken, b: &ProgramToken) -> Option<ProgramToken> {
  match (a, b) {
    (ProgramToken::ChangeAddr(a), ProgramToken::ChangeAddr(b)) => {
      Some(ProgramToken::ChangeAddr(a + b))
    }
    (ProgramToken::ChangeValue(a), ProgramToken::ChangeValue(b)) => {
      Some(ProgramToken::ChangeValue(a + b))
    }
    _ => None,
  }
}

fn optimize(tokens: Vec<ParseToken>) -> Vec<ProgramToken> {
  let program_tokens = tokens.into_iter().map(|token| match token {
    ParseToken::IncrAddr => ProgramToken::ChangeAddr(1),
    ParseToken::DecrAddr => ProgramToken::ChangeAddr(-1),
    ParseToken::IncrValue => ProgramToken::ChangeValue(1),
    ParseToken::DecrValue => ProgramToken::ChangeValue(-1),
    ParseToken::LoopStart => ProgramToken::LoopStart,
    ParseToken::LoopEnd => ProgramToken::LoopEnd,
    ParseToken::Print => ProgramToken::Print,
  });

  let without_duplicates = program_tokens.fuse_to_vec(try_fuse);
  without_duplicates
}

fn run_program(program: Vec<ProgramToken>) {
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
      Print => {
        print!("{}", memory[pointer as usize] as char);
      }
    }
    instruction_pointer += 1;
  }
}

fn main() {
  let hello_world = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
  let parsed_program = parse_program(hello_world);
  let optimized_program = optimize(parsed_program);
  println!("{:?}", optimized_program);
  run_program(optimized_program);
}
