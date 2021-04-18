use crate::lexer::Token;

pub trait Node {
  fn token_literal(&self) -> &str;
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Statement {
  node: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Expression {
  node: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Program {
  pub Statements: Vec<Statement>,
}

impl Program {
  pub fn new(v: Vec<Statement>) -> Program {
    Program { Statements: v }
  }
}

impl Node for Statement {
  fn token_literal(&self) -> &str {
    &self.node
  }
}

impl Node for Expression {
  fn token_literal(&self) -> &str {
    &self.node
  }
}

impl Node for Program {
  fn token_literal(&self) -> &str {
    if self.Statements.len() > 0 {
      &self.Statements[0].node
    } else {
      ""
    }
  }
}

pub struct Identifier {
  token: Token,
  value: String,
}

impl Node for Identifier {
  fn token_literal(&self) -> &str {
    &self.token.literal
  }
}

pub struct LetStatement {
  token: Token,
  name: Identifier,
  value: Expression,
}

impl Node for LetStatement {
  fn token_literal(&self) -> &str {
    &self.token.literal
  }
}
