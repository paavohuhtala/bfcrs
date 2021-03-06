extern crate bfcrs;

use bfcrs::types::ProgramToken;
use bfcrs::types::ProgramToken::*;

mod common;
use common::node_bridge::run_bf_in_node;
use common::test_util::{run_and_expect_same, run_and_expect_same_tokens};

#[test]
pub fn inc_dec_smoke_node() {
  let result = run_bf_in_node("++-- >+ >++");

  assert_eq!("", result.output);
  assert_eq!(&[0, 1, 2], &result.state.memory[0..3]);
  assert_eq!(2, result.state.pointer);
}

#[test]
pub fn inc_dec_smoke_same() {
  run_and_expect_same("++-- >+ >++");
}

#[test]
pub fn inc_dec_negative_1() {
  run_and_expect_same("-");
}

#[test]
pub fn inc_dec_negative_2() {
  run_and_expect_same(">-");
}

#[test]
pub fn zero_nop_1() {
  run_and_expect_same_tokens(&[
    ChangeAddr(100),
    ProgramToken::change_value(1),
    ProgramToken::set_value(0),
  ]);
}

#[test]
pub fn change_offset_smoke_1() {
  run_and_expect_same_tokens(&[ProgramToken::offs_change_value(1, 1)]);
}

#[test]
pub fn change_offset_smoke_2() {
  run_and_expect_same_tokens(&[ProgramToken::offs_change_value(1, -1)]);
}

#[test]
pub fn change_offset_nop_1() {
  run_and_expect_same_tokens(&[
    ProgramToken::offs_change_value(1, 1),
    ChangeAddr(1),
    ProgramToken::offs_change_value(0, -1),
  ]);
}

#[test]
pub fn change_offset_nop_2() {
  run_and_expect_same_tokens(&[
    ProgramToken::offs_change_value(1, 1),
    ProgramToken::offs_change_value(1, -1),
  ]);
}

#[test]
pub fn change_offset_nop_3() {
  run_and_expect_same_tokens(&[
    ProgramToken::offs_change_value(1, -1),
    ProgramToken::offs_change_value(1, 1),
  ]);
}

#[test]
pub fn change_offset_negative_offset() {
  run_and_expect_same_tokens(&[ChangeAddr(1), ProgramToken::offs_change_value(-1, 1)]);
}
