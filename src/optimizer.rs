use types::{ParseToken, ProgramToken};

fn merge_instructions(all_tokens: &Vec<ProgramToken>) -> Vec<ProgramToken> {
  let mut results: Vec<ProgramToken> = Vec::with_capacity(all_tokens.len());

  let mut tokens = all_tokens.as_slice();
  let mut prev = None;

  while tokens.len() > 0 || prev.is_some() {
    match (&prev, tokens) {
      (Some(ProgramToken::ChangeAddr(a)), [ProgramToken::ChangeAddr(b), tail..]) => {
        prev = Some(ProgramToken::ChangeAddr(a + b));
        tokens = tail;
      }
      (Some(ProgramToken::ChangeValue(a)), [ProgramToken::ChangeValue(b), tail..]) => {
        prev = Some(ProgramToken::ChangeValue(a + b));
        tokens = tail;
      }
      // Disabled until this is fixed in the WASM backend.
      /*(
        Some(ProgramToken::ChangeAddr(addr_offset_a)),
        [ProgramToken::ChangeValue(value), ProgramToken::ChangeAddr(addr_offset_b), tail..],
      ) if addr_offset_b + addr_offset_a == 0 =>
      {
        prev = Some(ProgramToken::ChangeOffset {
          addr_offset: *addr_offset_a,
          value: *value,
        });
        tokens = tail;
      }*/
      (Some(ProgramToken::Loop(body)), rest) => match body.as_slice() {
        &[ProgramToken::ChangeValue(x)] if x.abs() > 0 => {
          prev = Some(ProgramToken::Zero);
          tokens = rest;
        }
        _ => {
          results.push(ProgramToken::Loop(merge_instructions(body)));

          // This must be a separate check, because of pattern limitations.
          match rest {
            [] => {
              prev = None;
              tokens = &[];
            }
            [head, tail..] => {
              prev = Some(head.clone());
              tokens = tail;
            }
          }
        }
      },
      (Some(token), []) => {
        results.push(token.clone());
        prev = None;
      }
      (None, []) => {
        unreachable!();
      }
      (Some(ref token), [head, tail..]) => {
        results.push(token.clone());
        prev = Some(head.clone());
        tokens = tail;
      }
      (None, [head, tail..]) => {
        prev = Some(head.clone());
        tokens = tail;
      }
    }
  }

  results
}

pub fn optimize(tokens: Vec<ParseToken>) -> Vec<ProgramToken> {
  fn convert_tokens(offset: &mut usize, tokens: &Vec<ParseToken>, results: &mut Vec<ProgramToken>) {
    while *offset < tokens.len() {
      let token = &tokens[*offset];

      let next = match token {
        ParseToken::IncrAddr => ProgramToken::ChangeAddr(1),
        ParseToken::DecrAddr => ProgramToken::ChangeAddr(-1),
        ParseToken::IncrValue => ProgramToken::ChangeValue(1),
        ParseToken::DecrValue => ProgramToken::ChangeValue(-1),
        ParseToken::LoopStart => {
          let mut inner_body = Vec::new();
          *offset += 1;

          convert_tokens(offset, tokens, &mut inner_body);
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
  convert_tokens(&mut offset, &tokens, &mut program);

  if offset != tokens.len() {
    panic!("Malformed program.");
  }

  merge_instructions(&program)
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
  let after = merge_instructions(&before);

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
  let after = merge_instructions(&before);

  assert_eq!(&expected, &after);
}
