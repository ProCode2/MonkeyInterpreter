use crate::ast::{LetStatement, Program, Statement};
use crate::lexer::{Lexer, Token};
use crate::token::TokenType;

pub struct Parser {
  l: Lexer,
  cur_token: Token,
  peek_token: Token,
}

impl Parser {
  fn new(l: Lexer) -> Parser {
    let mut p = Parser {
      l: l,
      cur_token: Lexer::new_token(&TokenType::EOF, '\0'),
      peek_token: Lexer::new_token(&TokenType::EOF, '\0'),
    };
    p.next_token();
    p.next_token();
    p
  }

  fn next_token(&mut self) {
    self.cur_token = self.peek_token.clone();
    self.peek_token = self.l.next_token();
  }

  fn parse_program(&self) -> Program {
    let program = Program::new(Vec::new());
    while self.cur_token.token_type != &TokenType::EOF {
      let stmt = self.parse_statement();
      program.Statements.push(stmt);
      self.next_token();
    }
    program.clone()
  }

  fn parse_statement(&self) -> Statement {
    match self.cur_token.token_type {
      &TokenType::LET => self.parse_let_statement(),
      _ => Statement {
        node: "".to_string(),
      },
    }
  }
}
