use crate::lexer::Token;
// use crate::token::TokenType;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Statement {
  LetStatement(LetStatement),
  ReturnStatement(ReturnStatement),
  NilStatement,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Expression {
  Identifier(Identifier),
  NilExpression,
}

impl Statement {
  fn token_literal(&self) -> String {
    match self {
      Statement::LetStatement(ls) => ls.token.literal.clone(),
      Statement::ReturnStatement(rs) => rs.token.literal.clone(),
      _ => "".to_string(),
    }
  }
}

impl Expression {
  fn token_literal(&self) -> String {
    match self {
      Expression::Identifier(id) => id.token.literal.clone(),
      _ => "".to_string(),
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Program {
  pub statements: Vec<Statement>,
}

impl Program {
  pub fn new(v: Vec<Statement>) -> Program {
    Program { statements: v }
  }
  fn token_literal(&self) -> String {
    if self.statements.len() > 0 {
      self.statements[0].token_literal()
    } else {
      "".to_string()
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct LetStatement {
  pub token: Token,
  pub name: Expression,
  pub value: Expression,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ReturnStatement {
  pub token: Token,
  pub expression_value: Expression,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Identifier {
  pub token: Token,
  pub value: String,
}
