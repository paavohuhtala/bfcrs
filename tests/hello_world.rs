extern crate bfcrs;

mod common;
use common::node_bridge::run_bf;

#[test]
pub fn hello_world_compiler() {
  let result = run_bf(include_str!("../bf/hello.bf"));
  assert_eq!(include_str!("../bf/hello.bf.out"), result.output);
}
