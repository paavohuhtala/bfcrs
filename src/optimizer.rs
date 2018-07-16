use self::MemoryOp::*;
use self::ProgramToken::*;
use types::{MemoryOp, ParseToken, ProgramToken};

fn merge_instructions(all_tokens: &[ProgramToken]) -> Vec<ProgramToken> {
  let mut results: Vec<ProgramToken> = Vec::with_capacity(all_tokens.len());

  let mut prev = None;
  let mut tokens = all_tokens;

  while tokens.len() > 0 || prev.is_some() {
    let (new_prev, new_tokens): (Option<ProgramToken>, &[ProgramToken]) = match (&prev, tokens) {
      (Some(ChangeAddr(a)), [ChangeAddr(b), tail..]) => (Some(ChangeAddr(a + b)), tail),
      (Some(Offset(offs_a, ChangeValue(a))), [Offset(offs_b, ChangeValue(b)), tail..])
        if offs_a == offs_b =>
      {
        (Some(ProgramToken::offs_change_value(*offs_a, a + b)), tail)
      }
      (
        Some(ChangeAddr(addr_offset_a)),
        [Offset(0, ChangeValue(value)), ChangeAddr(addr_offset_b), tail..],
      )
        if addr_offset_a + addr_offset_b == 0 =>
      {
        (
          Some(ProgramToken::offs_change_value(*addr_offset_a, *value)),
          tail,
        )
      }
      (Some(Loop(body)), rest) => match body.as_slice() {
        &[Offset(0, ChangeValue(x))] if x.abs() > 0 => (Some(ProgramToken::set_value(0)), rest),
        _ => {
          results.push(Loop(merge_instructions(body)));

          // This must be a separate check, because of pattern limitations.
          match rest {
            [] => (None, &[]),
            [head, tail..] => (Some(head.clone()), tail),
          }
        }
      },
      (Some(Offset(offs_a, SetValue(0))), [Offset(offs_b, ChangeValue(a)), tail..])
        if offs_a == offs_b =>
      {
        (Some(ProgramToken::offs_set_value(*offs_a, *a)), tail)
      }
      (Some(Offset(offs_a, SetValue(_))), [Offset(offs_b, SetValue(a)), tail..])
        if offs_a == offs_b =>
      {
        (Some(ProgramToken::offs_set_value(*offs_a, *a)), tail)
      }
      (Some(token), []) => {
        results.push(token.clone());
        (None, &[])
      }
      (None, []) => unreachable!(),
      (Some(ref token), [head, tail..]) => {
        results.push(token.clone());
        (Some(head.clone()), tail)
      }
      (None, [head, tail..]) => (Some(head.clone()), tail),
    };

    prev = new_prev;
    tokens = new_tokens;
  }

  results
}

fn postpone_moves(all_tokens: &[ProgramToken]) -> Vec<ProgramToken> {
  let mut results: Vec<ProgramToken> = Vec::with_capacity(all_tokens.len());
  let mut i = 0;
  let mut offset = 0;

  while i < all_tokens.len() {
    match all_tokens[i] {
      ChangeAddr(addr) => {
        offset += addr;
      }
      Offset(addr_offset, ref op) => results.push(Offset(offset + addr_offset, op.clone())),
      Loop(ref inner) => {
        if offset != 0 {
          results.push(ChangeAddr(offset));
          offset = 0;
        }

        results.push(Loop(postpone_moves(&inner)));
      }
    }
    i += 1;
  }

  if offset != 0 {
    results.push(ChangeAddr(offset));
  }

  results
}

pub fn convert_tokens(all_tokens: &[ParseToken]) -> Vec<ProgramToken> {
  fn convert_tokens_rec(
    offset: &mut usize,
    tokens: &[ParseToken],
    results: &mut Vec<ProgramToken>,
  ) {
    while *offset < tokens.len() {
      let token = &tokens[*offset];

      let next = match token {
        ParseToken::IncrAddr => ChangeAddr(1),
        ParseToken::DecrAddr => ChangeAddr(-1),
        ParseToken::IncrValue => ProgramToken::change_value(1),
        ParseToken::DecrValue => ProgramToken::change_value(-1),
        ParseToken::Print => Offset(0, Print),
        ParseToken::LoopStart => {
          let mut inner_body = Vec::new();
          *offset += 1;

          convert_tokens_rec(offset, tokens, &mut inner_body);
          Loop(inner_body)
        }
        ParseToken::LoopEnd => {
          break;
        }
      };

      *offset += 1;
      results.push(next);
    }
  }

  let mut program = Vec::new();
  let mut offset = 0;
  convert_tokens_rec(&mut offset, &all_tokens, &mut program);

  if offset != all_tokens.len() {
    panic!("Malformed program.");
  }

  program
}

pub fn optimize(program: &[ProgramToken]) -> Vec<ProgramToken> {
  let mut tokens = program.to_vec();
  let mut iterations = 0;

  loop {
    println!("Optimization round: {}", iterations + 1);
    let optimized = postpone_moves(&merge_instructions(&tokens));
    if optimized == tokens {
      return optimized;
    }
    tokens = optimized;
    iterations += 1;
  }
}

pub fn optimize_parsed(tokens: &[ParseToken]) -> Vec<ProgramToken> {
  optimize(&convert_tokens(tokens))
}

#[test]
fn single_ops_are_maintained() {
  let before = vec![
    ChangeAddr(1),
    ProgramToken::change_value(1),
    ChangeAddr(1),
    ProgramToken::change_value(1),
  ];
  let expected = before.clone();
  let after = merge_instructions(&before);

  assert_eq!(&expected, &after);
}

#[test]
fn same_ops_are_merged() {
  let before = vec![
    ChangeAddr(1),
    ChangeAddr(1),
    ChangeAddr(1),
    ProgramToken::change_value(1),
    ProgramToken::change_value(1),
  ];
  let expected = vec![ChangeAddr(3), ProgramToken::change_value(2)];
  let after = merge_instructions(&before);

  assert_eq!(&expected, &after);
}
