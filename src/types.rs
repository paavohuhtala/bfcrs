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
  ChangeAddr(isize),
  ChangeValue { addr_offset: isize, value: i8 },
  SetValue { addr_offset: isize, value: i8 },
  Loop(Vec<ProgramToken>),
  Print,
}

impl ProgramToken {
  pub fn set_value(value: i8) -> ProgramToken {
    ProgramToken::SetValue {
      addr_offset: 0,
      value: value,
    }
  }
  pub fn offs_set_value(offset: isize, value: i8) -> ProgramToken {
    ProgramToken::SetValue {
      addr_offset: offset,
      value: value,
    }
  }
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
