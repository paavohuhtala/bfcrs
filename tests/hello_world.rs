extern crate bfcrs;
extern crate byteorder;

use bfcrs::compile_program;

mod common;
use common::node_bridge::NodeBridge;

#[test]
pub fn hello_world_compiler() {
  let source = compile_program(include_str!("../bf/hello.bf"));

  let mut client = NodeBridge::create();
  client.send_message(&source);

  let output = client.read_message_str();
  assert_eq!(include_str!("../bf/hello.bf.out"), output);
}
