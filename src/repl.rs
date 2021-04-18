use crate::lexer::Lexer;
use std::io::{stdin, stdout, Write};

pub fn start() {
  let prompt = ">> ";
  let mut input = String::new();
  loop {
    print!("{}", prompt);
    stdout().flush().expect("Something went wrong");
    stdin().read_line(&mut input).unwrap();
    if input.trim() == "exit" {
      break;
    }
    let mut l = Lexer::new(input.clone());
    while l.peek_char() != '\0' {
      println!("{:?}", l.next_token())
    }
    input = "".to_string();
  }
}
