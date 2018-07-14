extern crate byteorder;

use std::io::{Read, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use self::byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};

use bfcrs::compile_program;
use bfcrs::types::State;

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

  pub fn read_state(&mut self) -> State {
    let bytes = self.read_message();
    let byte_slice = &bytes;
    let (pointer_buf, memory) = byte_slice.split_at(4);
    let pointer = LittleEndian::read_u32(pointer_buf) as usize;

    State {
      pointer,
      memory: memory.to_vec(),
    }
  }
}

pub struct RunResult {
  pub output: String,
  pub state: State,
}

pub fn run_wasm(code: &[u8]) -> RunResult {
  let mut bridge = NodeBridge::create();
  bridge.send_message(&code);

  let output = bridge.read_message_str();
  let state = bridge.read_state();

  // Send something so that Node knows we are done.
  bridge.send_message(&[1]);

  RunResult { output, state }
}

pub fn run_bf(source: &str) -> RunResult {
  let code = compile_program(source);
  run_wasm(&code)
}
