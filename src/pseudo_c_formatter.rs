use types::ProgramToken;

pub fn print_as_c(program: &[ProgramToken], indent: String) {
  for token in program {
    match token {
      ProgramToken::ChangeAddr(offset) => {
        println!("{}pointer += {}", indent, offset);
      }
      ProgramToken::ChangeValue { addr_offset, value } => {
        println!("{}memory[pointer + {}] += {}", indent, addr_offset, value);
      }
      ProgramToken::SetValue { addr_offset, value } => {
        println!("{}memory[pointer + {}] = {}", indent, addr_offset, value);
      }
      ProgramToken::Loop(inner) => {
        println!("{}while (memory[pointer]) {{", indent);
        print_as_c(inner, indent.clone() + "  ");
        println!("{}}}", indent);
      }
      ProgramToken::Print => {
        println!("{}print(memory[pointer])", indent);
      }
    }
  }
}
