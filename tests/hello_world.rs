extern crate bfcrs;

mod common;
use common::interpreter_util::run_bf_in_interpreter;
use common::node_bridge::run_bf_in_node;
use common::test_util::run_and_expect_same;

#[test]
pub fn hello_world_compiler_output() {
  let result = run_bf_in_node(include_str!("../bf/hello.bf"));
  assert_eq!(include_str!("../bf/hello.bf.out"), result.output);
}

#[test]
pub fn hello_world_interpreter_output() {
  let result = run_bf_in_interpreter(include_str!("../bf/hello.bf"));
  assert_eq!(include_str!("../bf/hello.bf.out"), result.output);
}

#[test]
pub fn hello_world_state() {
  run_and_expect_same(include_str!("../bf/hello.bf"));
}
