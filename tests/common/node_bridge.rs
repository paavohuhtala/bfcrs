use std::io::{Read, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub struct NodeBridge {
  stdin: ChildStdin,
  stdout: ChildStdout,
}

impl NodeBridge {
  pub fn create() -> NodeBridge {
    let server = Command::new("node")
      .args(&["./index.js"])
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .current_dir("./wasm_test_runner")
      .spawn()
      .expect("Process should be able to be spawned.");

    let Child { stdin, stdout, .. } = server;

    let client = NodeBridge {
      stdin: stdin.unwrap(),
      stdout: stdout.unwrap(),
    };

    client
  }

  pub fn send_message(&mut self, data: &[u8]) {
    self
      .stdin
      .write_u32::<LittleEndian>(data.len() as u32)
      .unwrap();
    self.stdin.write(data).unwrap();
  }

  pub fn read_message(&mut self) -> Vec<u8> {
    let len = self.stdout.by_ref().read_u32::<LittleEndian>().unwrap();
    let mut buffer = vec![0u8; len as usize];
    self.stdout.read_exact(&mut buffer).unwrap();
    buffer
  }

  pub fn read_message_str(&mut self) -> String {
    let buffer = self.read_message();
    String::from_utf8(buffer).unwrap()
  }
}
