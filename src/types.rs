#[derive(Debug, PartialEq)]
pub enum ParseToken {
  IncrAddr,
  DecrAddr,
  IncrValue,
  DecrValue,
  LoopStart,
  LoopEnd,
  Print,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProgramToken {
  ChangeValue(i8),
  ChangeAddr(isize),
  ChangeOffset { addr_offset: isize, value: i8 },
  Zero,
  Loop(Vec<ProgramToken>),
  Print,
}

pub struct State {
  pub pointer: usize,
  pub memory: Vec<u8>,
}

impl State {
  pub fn new() -> State {
    State {
      pointer: 0,
      memory: vec![0u8; 65536],
    }
  }
}
