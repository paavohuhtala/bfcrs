extern crate bfcrs;

mod common;
use common::node_bridge::run_bf_in_node;
use common::test_util::run_and_expect_same;

#[test]
pub fn hello_world_wasm_output() {
  let result = run_bf_in_node(include_str!("../bf/hello.bf"));
  assert_eq!(include_str!("../bf/hello.bf.out"), result.output);
}

#[test]
pub fn hello_world_same() {
  run_and_expect_same(include_str!("../bf/hello.bf"));
}

#[test]
pub fn sierpinski_wasm_output() {
  let result = run_bf_in_node(include_str!("../bf/sierpinski.bf"));
  assert_eq!(include_str!("../bf/sierpinski.bf.out"), result.output);
}

#[test]
pub fn sierpinski_same() {
  run_and_expect_same(include_str!("../bf/sierpinski.bf"));
}

#[test]
pub fn mandelbrot_wasm_output() {
  let result = run_bf_in_node(include_str!("../bf/mandelbrot.bf"));
  assert_eq!(include_str!("../bf/mandelbrot.bf.out"), result.output);
}
