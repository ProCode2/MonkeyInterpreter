mod lexer;
mod token;

use lexer::Lexer;
use token::TokenType;

fn main() {
    let input = "(5 == (6-1))".to_string();
    println!("{}", input);
    let mut l = Lexer::new(input.clone());
    while l.peek_char() != '\0' {
        println!("{:?}", l.next_token());
    }
    println!("Hello, world!");
}
