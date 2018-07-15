use bfcrs::optimizer::optimize_parsed;
use bfcrs::parser::parse_program;
use common::interpreter_util::run_tokens_in_interpreter;
use common::node_bridge::run_tokens_in_node;

pub fn chunked_assert_eq(a: &[u8], b: &[u8]) {
  assert_eq!(a.len(), b.len());
  let chunk_size = 128;

  for (i, (chunk_a, chunk_b)) in a.chunks(chunk_size).zip(b.chunks(chunk_size)).enumerate() {
    assert_eq!(chunk_a, chunk_b, "Starting at offset {}", i * chunk_size);
  }
}

pub fn run_and_expect_same(source: &str) {
  let program = optimize_parsed(&parse_program(source));
  let interpreter_result = run_tokens_in_interpreter(&program);
  let node_result = run_tokens_in_node(&program);

  assert_eq!(
    interpreter_result.output, interpreter_result.output,
    "Output should equal."
  );
  assert_eq!(
    interpreter_result.state.pointer, interpreter_result.state.pointer,
    "Pointers should equal."
  );

  chunked_assert_eq(&interpreter_result.state.memory, &node_result.state.memory);
}
