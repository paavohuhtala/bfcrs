use types::{MemoryOp, ProgramToken};

pub fn print_as_c(program: &[ProgramToken], indent: String) {
  for token in program {
    match token {
      ProgramToken::ChangeAddr(offset) => {
        println!("{}pointer += {}", indent, offset);
      }
      ProgramToken::Offset(offset, MemoryOp::ChangeValue(value)) => {
        println!("{}memory[pointer + {}] += {}", indent, offset, value);
      }
      ProgramToken::Offset(offset, MemoryOp::SetValue(value)) => {
        println!("{}memory[pointer + {}] = {}", indent, offset, value);
      }
      ProgramToken::Offset(offset, MemoryOp::Print) => {
        println!("{}print(memory[pointer + {}])", indent, offset);
      }
      ProgramToken::Loop(inner) => {
        println!("{}while (memory[pointer]) {{", indent);
        print_as_c(inner, indent.clone() + "  ");
        println!("{}}}", indent);
      }
    }
  }
}
