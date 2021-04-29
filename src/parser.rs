use crate::ast::*;
use crate::lexer::{Lexer, Token};
use crate::token::TokenType;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Parser {
  l: Lexer,
  cur_token: Token,
  peek_token: Token,
  errors: Vec<String>,
}

impl Parser {
  pub fn new(l: Lexer) -> Parser {
    let mut p = Parser {
      l: l,
      cur_token: Lexer::new_token(&TokenType::EOF, '\0'),
      peek_token: Lexer::new_token(&TokenType::EOF, '\0'),
      errors: Vec::new(),
    };
    p.next_token();
    p.next_token();
    p
  }

  pub fn errors(&self) -> Vec<String> {
    self.errors.clone()
  }

  pub fn peek_error(&mut self, t: &TokenType) {
    let msg = String::from(format!(
      "Expected next token to be {:?} but found {:?} instead.",
      t, self.peek_token.token_type
    ));

    self.errors.push(msg);
  }

  pub fn next_token(&mut self) {
    self.cur_token = self.peek_token.clone();
    self.peek_token = self.l.next_token();
  }

  pub fn parse_program(&mut self) -> Program {
    let mut program = Program::new(Vec::new());
    while self.cur_token.token_type != &TokenType::EOF {
      let stmt = self.parse_statement();
      program.statements.push(stmt);
      self.next_token();
    }
    program.clone()
  }

  pub fn parse_statement(&mut self) -> Statement {
    match self.cur_token.token_type {
      &TokenType::LET => self.parse_let_statement(),
      &TokenType::RETURN => self.parse_return_statement(),
      _ => Statement::NilStatement,
    }
  }

  pub fn parse_return_statement(&mut self) -> Statement {
    let stmt = ReturnStatement {
      token: self.cur_token.clone(),
      expression_value: Expression::NilExpression,
    };

    self.next_token();
    // TODO: SKIPPING PARSING EXPRESSION
    while !self.cur_token_is(TokenType::SEMICOLON) {
      self.next_token();
    }
    Statement::ReturnStatement(stmt)
  }

  pub fn parse_let_statement(&mut self) -> Statement {
    let mut stmt = LetStatement {
      token: self.cur_token.clone(),
      name: Expression::Identifier(Identifier {
        token: self.cur_token.clone(),
        value: self.cur_token.literal.clone(),
      }),
      value: Expression::NilExpression,
    };

    if !self.expect_peek(TokenType::IDENT) {
      return Statement::NilStatement;
    } else {
      stmt.name = Expression::Identifier(Identifier {
        token: self.cur_token.clone(),
        value: self.cur_token.literal.clone(),
      });
    }
    if !self.expect_peek(TokenType::ASSIGN) {
      return Statement::NilStatement;
    } else {
      // TODO: SKIPPING PARSING EXPRESSION
      while !self.cur_token_is(TokenType::SEMICOLON) {
        self.next_token();
      }
      return Statement::LetStatement(stmt);
    }
  }

  pub fn cur_token_is(&self, t: TokenType) -> bool {
    self.cur_token.token_type == &t
  }

  pub fn peek_token_is(&self, t: &TokenType) -> bool {
    self.peek_token.token_type == t
  }

  pub fn expect_peek(&mut self, t: TokenType) -> bool {
    if self.peek_token_is(&t) {
      self.next_token();
      true
    } else {
      self.peek_error(&t);
      false
    }
  }
}

type parse_fn = fn() -> ast::Expression
