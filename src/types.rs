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
pub enum MemoryOp {
  ChangeValue(i8),
  SetValue(i8),
  Print,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProgramToken {
  ChangeAddr(isize),
  Offset(isize, MemoryOp),
  Loop(Vec<ProgramToken>),
}

impl ProgramToken {
  pub fn change_value(value: i8) -> ProgramToken {
    ProgramToken::Offset(0, MemoryOp::ChangeValue(value))
  }

  pub fn offs_change_value(offset: isize, value: i8) -> ProgramToken {
    ProgramToken::Offset(offset, MemoryOp::ChangeValue(value))
  }

  pub fn set_value(value: i8) -> ProgramToken {
    ProgramToken::Offset(0, MemoryOp::SetValue(value))
  }

  pub fn offs_set_value(offset: isize, value: i8) -> ProgramToken {
    ProgramToken::Offset(offset, MemoryOp::SetValue(value))
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
