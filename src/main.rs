mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    let input = "return 10;
    return 10;
    return 838383;
    "
    .to_string();
    // repl::start();

    let l = lexer::Lexer::new(input);
    let mut p = parser::Parser::new(l);
    let program = p.parse_program();
    println!("{:?}", program.statements);
}
