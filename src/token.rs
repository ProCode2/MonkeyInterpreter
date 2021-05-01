use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum TokenType {
  ILLEGAL,
  EOF,

  // identifiers + literals
  IDENT, // add, foobar, x, y, ...
  INT,
  // Operators
  ASSIGN,
  PLUS,
  MINUS,
  BANG,
  ASTERISK,
  SLASH,
  LT,
  GT,
  // Delimiters
  COMMA,
  SEMICOLON,
  LPAREN,
  RPAREN,
  LBRACE,
  RBRACE,
  // Keywords
  // 1343456
  FUNCTION,
  LET,
  TRUE,
  FALSE,
  IF,
  ELSE,
  RETURN,
  EQ,
  NOTEQ,
}

pub enum Statement {
  LetStatement,
}

// impl PartialEq<Token> for TokenType {
//   fn eq(&self, other: &Token) -> bool {
//     self == other.token_type
//   }
// }

fn get_keywords() -> HashMap<String, &'static TokenType> {
  let mut keywords: HashMap<String, &TokenType> = HashMap::new();
  keywords.insert(String::from("fn"), &TokenType::FUNCTION);
  keywords.insert(String::from("let"), &TokenType::LET);
  keywords.insert(String::from("true"), &TokenType::TRUE);
  keywords.insert(String::from("false"), &TokenType::FALSE);
  keywords.insert(String::from("if"), &TokenType::IF);
  keywords.insert(String::from("else"), &TokenType::ELSE);
  keywords.insert(String::from("return"), &TokenType::RETURN);
  keywords
}

pub fn lookup_ident(ident: &String) -> &'static TokenType {
  let keywords = get_keywords();
  keywords.get(ident).unwrap_or(&&TokenType::IDENT)
}
