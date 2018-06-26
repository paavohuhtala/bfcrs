#![feature(iterator_find_map)]

use std::num::Wrapping;

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

enum ProgramToken {
  ChangeValue(i32),
  ChangeAddress(i32),
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

fn find_within_indices<T: PartialEq>(haystack: &Vec<T>, indices: impl IntoIterator<Item = usize>, needle: &T) -> Option<usize> {
  for i in indices {
    match haystack.get(i) {
      None => {
        return None;
      },
      Some(x) if x == needle => {
        return Some(i);
      },
      Some(_) => { }
    }
  }

  return None;
}

fn find_index<T: PartialEq>(haystack: &Vec<T>, starting_offset: usize, needle: &T) -> Option<usize> {
  find_within_indices(haystack, starting_offset..haystack.len(), needle)
}

fn find_index_backwards<T: PartialEq>(haystack: &Vec<T>, starting_offset: usize, needle: &T) -> Option<usize> {
  find_within_indices(haystack, (0..starting_offset).rev(), needle)
}

fn run_program(program: Vec<ParseToken>) {
  let mut memory = vec![Wrapping(0u8); std::u16::MAX as usize];
  let mut pointer = 0;
  let mut instruction_pointer = 0;

  while let Some(op) = program.get(instruction_pointer) {
    match op {
      ParseToken::IncrAddr => pointer += 1,
      ParseToken::DecrAddr => pointer -= 1,
      ParseToken::IncrValue => memory[pointer] += Wrapping(1),
      ParseToken::DecrValue => memory[pointer] -= Wrapping(1),
      ParseToken::LoopStart => {
        if memory[pointer] == Wrapping(0) {
          if let Some(offset) = find_index(&program, instruction_pointer, &ParseToken::LoopEnd) {
            instruction_pointer = offset + 1;
            continue;
          } else {
            panic!("Unmatched '['");
          }
        }
      },
      ParseToken::LoopEnd => {
        if memory[pointer] != Wrapping(0) {
          if let Some(offset) = find_index_backwards(&program, instruction_pointer, &ParseToken::LoopStart) {
          instruction_pointer = offset + 1;
          continue;
        } else {
          panic!("Unmatched ']'");
        }
      }
      },
      ParseToken::Print => {
        print!("{}", memory[pointer].0 as char);
      }
    }
    instruction_pointer += 1;
  }
}

fn main() {
  let hello_world = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
  let parsed_program = parse_program(hello_world);
  run_program(parsed_program);
}
