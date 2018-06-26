#![feature(iterator_find_map)]

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

#[derive(Debug, PartialEq)]
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

fn optimize(tokens: Vec<ParseToken>) -> Vec<ProgramToken> {
  let mut program = Vec::new();

  for token in tokens {
    program.push(match token {
      ParseToken::IncrAddr => ProgramToken::ChangeAddr(1),
      ParseToken::DecrAddr => ProgramToken::ChangeAddr(-1),
      ParseToken::IncrValue => ProgramToken::ChangeValue(1),
      ParseToken::DecrValue => ProgramToken::ChangeValue(-1),
      ParseToken::LoopStart => ProgramToken::LoopStart,
      ParseToken::LoopEnd => ProgramToken::LoopEnd,
      ParseToken::Print => ProgramToken::Print
    });
  }

  program
}

fn run_program(program: Vec<ProgramToken>) {
  let mut memory = vec![0u8; std::u16::MAX as usize];
  let mut pointer: u16 = 0;
  let mut instruction_pointer = 0;

  use ProgramToken::*;

  while let Some(op) = program.get(instruction_pointer) {
    match op {
      ChangeAddr(by) => pointer = by.wrapping_add(pointer as i32) as u16,
      ChangeValue(by) => memory[pointer as usize] = by.wrapping_add(memory[pointer as usize] as i32) as u8,
      LoopStart => {
        if memory[pointer as usize] == 0 {
          if let Some(offset) = find_index(&program, instruction_pointer, &LoopEnd) {
            instruction_pointer = offset + 1;
            continue;
          } else {
            panic!("Unmatched '['");
          }
        }
      },
      LoopEnd => {
        if memory[pointer as usize] != 0 {
          if let Some(offset) = find_index_backwards(&program, instruction_pointer, &LoopStart) {
          instruction_pointer = offset + 1;
          continue;
        } else {
          panic!("Unmatched ']'");
        }
      }
      },
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
  run_program(optimized_program);
}
