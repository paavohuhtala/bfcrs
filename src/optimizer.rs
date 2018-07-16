use types::{ParseToken, ProgramToken};

fn merge_instructions(all_tokens: &[ProgramToken]) -> Vec<ProgramToken> {
  let mut results: Vec<ProgramToken> = Vec::with_capacity(all_tokens.len());

  let mut prev = None;
  let mut tokens = all_tokens;

  while tokens.len() > 0 || prev.is_some() {
    let (new_prev, new_tokens): (Option<ProgramToken>, &[ProgramToken]) = match (&prev, tokens) {
      (Some(ProgramToken::ChangeAddr(a)), [ProgramToken::ChangeAddr(b), tail..]) => {
        (Some(ProgramToken::ChangeAddr(a + b)), tail)
      }
      (
        Some(ProgramToken::ChangeValue {
          addr_offset: offs_a,
          value: a,
        }),
        [ProgramToken::ChangeValue {
          addr_offset: offs_b,
          value: b,
        }, tail..],
      )
        if offs_a == offs_b =>
      {
        (
          Some(ProgramToken::ChangeValue {
            addr_offset: *offs_a,
            value: a + b,
          }),
          tail,
        )
      }
      (
        Some(ProgramToken::ChangeAddr(addr_offset_a)),
        [ProgramToken::ChangeValue {
          addr_offset: 0,
          value,
        }, ProgramToken::ChangeAddr(addr_offset_b), tail..],
      )
        if addr_offset_a + addr_offset_b == 0 =>
      {
        (
          Some(ProgramToken::ChangeValue {
            addr_offset: *addr_offset_a,
            value: *value,
          }),
          tail,
        )
      }
      (Some(ProgramToken::Loop(body)), rest) => match body.as_slice() {
        &[ProgramToken::ChangeValue {
          addr_offset: 0,
          value: x,
        }]
          if x.abs() > 0 =>
        {
          (Some(ProgramToken::SetValue(0)), rest)
        }
        _ => {
          results.push(ProgramToken::Loop(merge_instructions(body)));

          // This must be a separate check, because of pattern limitations.
          match rest {
            [] => (None, &[]),
            [head, tail..] => (Some(head.clone()), tail),
          }
        }
      },
      (
        Some(ProgramToken::SetValue(0)),
        [ProgramToken::ChangeValue {
          addr_offset: 0,
          value: a,
        }, tail..],
      ) => (Some(ProgramToken::SetValue(*a)), tail),
      (Some(ProgramToken::SetValue(_)), [ProgramToken::SetValue(a), tail..]) => {
        (Some(ProgramToken::SetValue(*a)), tail)
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

pub fn convert_tokens(all_tokens: &[ParseToken]) -> Vec<ProgramToken> {
  fn convert_tokens_rec(
    offset: &mut usize,
    tokens: &[ParseToken],
    results: &mut Vec<ProgramToken>,
  ) {
    while *offset < tokens.len() {
      let token = &tokens[*offset];

      let next = match token {
        ParseToken::IncrAddr => ProgramToken::ChangeAddr(1),
        ParseToken::DecrAddr => ProgramToken::ChangeAddr(-1),
        ParseToken::IncrValue => ProgramToken::ChangeValue {
          addr_offset: 0,
          value: 1,
        },
        ParseToken::DecrValue => ProgramToken::ChangeValue {
          addr_offset: 0,
          value: -1,
        },
        ParseToken::LoopStart => {
          let mut inner_body = Vec::new();
          *offset += 1;

          convert_tokens_rec(offset, tokens, &mut inner_body);
          ProgramToken::Loop(inner_body)
        }
        ParseToken::LoopEnd => {
          break;
        }
        ParseToken::Print => ProgramToken::Print,
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
  merge_instructions(program)
}

pub fn optimize_parsed(tokens: &[ParseToken]) -> Vec<ProgramToken> {
  optimize(&convert_tokens(tokens))
}

#[test]
fn single_ops_are_maintained() {
  let before = vec![
    ProgramToken::ChangeAddr(1),
    ProgramToken::ChangeValue {
      addr_offset: 0,
      value: 1,
    },
    ProgramToken::ChangeAddr(1),
    ProgramToken::ChangeValue {
      addr_offset: 0,
      value: 1,
    },
  ];
  let expected = before.clone();
  let after = merge_instructions(&before);

  assert_eq!(&expected, &after);
}

#[test]
fn same_ops_are_merged() {
  let before = vec![
    ProgramToken::ChangeAddr(1),
    ProgramToken::ChangeAddr(1),
    ProgramToken::ChangeAddr(1),
    ProgramToken::ChangeValue {
      addr_offset: 0,
      value: 1,
    },
    ProgramToken::ChangeValue {
      addr_offset: 0,
      value: 1,
    },
  ];
  let expected = vec![
    ProgramToken::ChangeAddr(3),
    ProgramToken::ChangeValue {
      addr_offset: 0,
      value: 2,
    },
  ];
  let after = merge_instructions(&before);

  assert_eq!(&expected, &after);
}
