use ParseToken;
use ProgramToken;

use utils::iter_utils::IterUtils;

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

fn remove_duplicates(tokens: impl Iterator<Item = ProgramToken>) -> Vec<ProgramToken> {
  tokens.fuse_to_vec(try_fuse)
}

pub fn optimize(tokens: Vec<ParseToken>) -> Vec<ProgramToken> {
  let program_tokens = tokens.into_iter().map(|token| match token {
    ParseToken::IncrAddr => ProgramToken::ChangeAddr(1),
    ParseToken::DecrAddr => ProgramToken::ChangeAddr(-1),
    ParseToken::IncrValue => ProgramToken::ChangeValue(1),
    ParseToken::DecrValue => ProgramToken::ChangeValue(-1),
    ParseToken::LoopStart => ProgramToken::LoopStart,
    ParseToken::LoopEnd => ProgramToken::LoopEnd,
    ParseToken::Print => ProgramToken::Print,
  });

  let without_duplicates = remove_duplicates(program_tokens);
  without_duplicates
}

#[test]
fn single_ops_are_maintained() {
  let before = vec![
    ProgramToken::ChangeAddr(1),
    ProgramToken::ChangeValue(1),
    ProgramToken::ChangeAddr(1),
    ProgramToken::ChangeValue(1),
  ];
  let expected = before.clone();
  let after = remove_duplicates(before.into_iter());

  assert_eq!(&expected, &after);
}

#[test]
fn same_ops_are_merged() {
  let before = vec![
    ProgramToken::ChangeAddr(1),
    ProgramToken::ChangeAddr(1),
    ProgramToken::ChangeAddr(1),
    ProgramToken::ChangeValue(1),
    ProgramToken::ChangeValue(1),
  ];
  let expected = vec![ProgramToken::ChangeAddr(3), ProgramToken::ChangeValue(2)];
  let after = remove_duplicates(before.into_iter());

  assert_eq!(&expected, &after);
}
