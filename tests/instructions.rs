extern crate bfcrs;

mod common;
use common::node_bridge::run_bf_in_node;

#[test]
pub fn inc_dec_smoke() {
  let result = run_bf_in_node("++-- >+ >++");

  assert_eq!("", result.output);
  assert_eq!(&[0, 1, 2], &result.state.memory[0..3]);
  assert_eq!(2, result.state.pointer);
}
