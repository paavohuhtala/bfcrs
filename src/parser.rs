use types::ParseToken;

pub fn parse_program(program: &str) -> Vec<ParseToken> {
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
