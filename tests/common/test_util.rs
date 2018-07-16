use bfcrs::optimizer::optimize_parsed;
use bfcrs::parser::parse_program;
use bfcrs::types::ProgramToken;
use common::interpreter_util::run_tokens_in_interpreter;
use common::node_bridge::run_tokens_in_node;
use common::types::RunResult;

pub fn chunked_assert_eq(a: &[u8], b: &[u8]) {
  assert_eq!(a.len(), b.len());
  let chunk_size = 64;

  for (i, (chunk_a, chunk_b)) in a.chunks(chunk_size).zip(b.chunks(chunk_size)).enumerate() {
    assert_eq!(chunk_a, chunk_b, "Starting at offset {}", i * chunk_size);
  }
}

pub fn compare_results(a: &RunResult, b: &RunResult) {
  chunked_assert_eq(&a.state.memory, &b.state.memory);

  assert_eq!(a.state.pointer, b.state.pointer, "Pointers should equal.");
  assert_eq!(a.output, b.output, "Output should equal.");
}

pub fn run_and_expect_same_tokens(program: &[ProgramToken]) {
  let interpreter_result = run_tokens_in_interpreter(&program);
  let node_result = run_tokens_in_node(&program);
  compare_results(&interpreter_result, &node_result);
}

pub fn run_and_expect_same(source: &str) {
  let program = optimize_parsed(&parse_program(source));
  run_and_expect_same_tokens(&program);
}
