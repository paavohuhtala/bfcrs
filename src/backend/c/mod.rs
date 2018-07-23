use std::io::Write;

use backend::Backend;
use types::MemoryOp::*;
use types::ProgramToken;

pub struct CBackend;

impl Backend for CBackend {
  fn extension(&self) -> &'static str {
    ".c"
  }

  fn compile_to_stream(&self, tokens: &[ProgramToken], stream: &mut dyn Write) {
    let mut output = String::new();

    use std::fmt::Write;

    write!(&mut output, "#include <stdlib.h>\n").unwrap();
    write!(&mut output, "#include <stdio.h>\n").unwrap();
    write!(&mut output, "int main() {{\n").unwrap();
    write!(&mut output, "  char* buffer = malloc(30000);\n").unwrap();
    write!(&mut output, "  int pointer = 0;\n").unwrap();

    fn compile_tokens(mut output: &mut String, tokens: &[ProgramToken], indent: String) {
      for token in tokens {
        match token {
          ProgramToken::ChangeAddr(offset) => {
            write!(&mut output, "{}pointer += {};\n", indent, offset).unwrap();
          }
          ProgramToken::Offset(offset, ChangeValue(value)) => {
            write!(
              &mut output,
              "{}buffer[pointer + {}] += {};\n",
              indent, offset, value
            ).unwrap();
          }
          ProgramToken::Offset(offset, SetValue(value)) => {
            write!(
              &mut output,
              "{}buffer[pointer + {}] = {};\n",
              indent, offset, value
            ).unwrap();
          }
          ProgramToken::Offset(offset, Print) => {
            write!(
              &mut output,
              "{}putchar(buffer[pointer + {}]);\n",
              indent, offset
            ).unwrap();
          }
          ProgramToken::Loop(inner) => {
            write!(&mut output, "{}while (buffer[pointer]) {{\n", indent).unwrap();
            compile_tokens(&mut output, inner, indent.clone() + "  ");
            write!(&mut output, "{}}}\n", indent).unwrap();
          }
        }
      }
    }

    compile_tokens(&mut output, tokens, "  ".to_string());

    write!(&mut output, "  free(buffer);\n").unwrap();
    write!(&mut output, "  return 0;\n").unwrap();
    write!(&mut output, "}}\n").unwrap();

    stream.write(output.as_bytes()).unwrap();
  }
}
