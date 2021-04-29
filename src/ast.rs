use crate::lexer::Token;
// use crate::token::TokenType;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Statement {
  LetStatement(LetStatement),
  ReturnStatement(ReturnStatement),
  ExpressionStatement(ExpressionStatement),
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
      Statement::ExpressionStatement(es) => es.token.literal.clone(),
      _ => "".to_string(),
    }
  }

  fn string(&self) -> String {
    let mut statement = String::from("");
    match self {
      Statement::LetStatement(ls) => {
        statement.push_str(&format!(
          "{} {} = {};",
          &ls.token.literal,
          ls.name.string(),
          ls.value.string()
        ));
      }
      Statement::ReturnStatement(rs) => {
        statement.push_str(&format!(
          "{} {};",
          &rs.token.literal,
          &rs.expression_value.string()
        ));
      }
      Statement::ExpressionStatement(es) => {
        statement.push_str(&format!("{}", &es.expression.string(),));
      }
      Statement::NilStatement => statement.push_str(""),
    }
    statement
  }
}

impl Expression {
  fn token_literal(&self) -> String {
    match self {
      Expression::Identifier(id) => id.token.literal.clone(),
      _ => "".to_string(),
    }
  }

  fn string(&self) -> String {
    let mut statement = String::from("");
    match self {
      Expression::Identifier(i) => statement.push_str(&i.value),
      Expression::NilExpression => statement.push_str(""),
    }
    statement
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
  pub fn string(&self) -> String {
    let mut statements = String::from("");
    for statement in &self.statements {
      statements.push_str(&format!("{}\n", &statement.string()))
    }
    statements
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ExpressionStatement {
  pub token: Token,
  pub expression: Expression,
}
