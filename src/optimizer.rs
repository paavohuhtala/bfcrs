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

  let without_duplicates = program_tokens.fuse_to_vec(try_fuse);
  without_duplicates
}
