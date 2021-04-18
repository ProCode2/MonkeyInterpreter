use crate::token::{lookup_ident, TokenType};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
  pub token_type: &'static TokenType,
  pub literal: String,
}

// impl PartialEq<TokenType> for Token {
//   fn eq(&self, other: &TokenType) -> bool {
//     self.token_type == other
//   }
// }

pub struct Lexer {
  input: String,
  position: usize,
  read_position: usize, // for storing next character
  ch: char,             // current char
}

impl Lexer {
  pub fn new(input: String) -> Lexer {
    let mut new_lexer = Lexer {
      input: input,
      position: 0,
      read_position: 0,
      ch: '\0',
    };
    new_lexer.read_char();
    new_lexer
  }

  // see whats next
  pub fn peek_char(&self) -> char {
    if self.read_position >= self.input.len() {
      '\0'
    } else {
      self.input.chars().nth(self.read_position).unwrap_or('\0')
    }
  }

  fn read_char(&mut self) {
    if self.read_position >= self.input.len() {
      self.ch = '\0';
    } else {
      self.ch = self.input.chars().nth(self.read_position).unwrap();
    }
    self.position = self.read_position;
    self.read_position += 1;
  }

  pub fn new_token(token_type: &'static TokenType, ch: char) -> Token {
    Token {
      token_type: token_type,
      literal: ch.to_string(),
    }
  }

  fn skip_whitespace(&mut self) {
    while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
      self.read_char();
    }
  }

  pub fn next_token(&mut self) -> Token {
    let mut token = Lexer::new_token(&TokenType::EOF, '\0');
    self.skip_whitespace();
    match self.ch {
      '=' => {
        if self.peek_char() == '=' {
          let ch = self.ch;
          self.read_char();
          let lit = format!("{}{}", ch, self.ch);
          token = Token {
            token_type: &TokenType::EQ,
            literal: lit,
          }
        } else {
          token = Lexer::new_token(&TokenType::ASSIGN, self.ch)
        }
      }
      '+' => token = Lexer::new_token(&TokenType::PLUS, self.ch),
      '-' => token = Lexer::new_token(&TokenType::MINUS, self.ch),
      '!' => {
        if self.peek_char() == '=' {
          let ch = self.ch;
          self.read_char();
          let lit = format!("{}{}", ch, self.ch);
          token = Token {
            token_type: &TokenType::NOTEQ,
            literal: lit,
          }
        } else {
          token = Lexer::new_token(&TokenType::BANG, self.ch)
        }
      }
      '/' => token = Lexer::new_token(&TokenType::SLASH, self.ch),
      '*' => token = Lexer::new_token(&TokenType::ASTERISK, self.ch),
      '<' => token = Lexer::new_token(&TokenType::LT, self.ch),
      '>' => token = Lexer::new_token(&TokenType::GT, self.ch),
      ';' => token = Lexer::new_token(&TokenType::SEMICOLON, self.ch),
      '(' => token = Lexer::new_token(&TokenType::LPAREN, self.ch),
      ')' => token = Lexer::new_token(&TokenType::RPAREN, self.ch),
      ',' => token = Lexer::new_token(&TokenType::COMMA, self.ch),
      '{' => token = Lexer::new_token(&TokenType::LBRACE, self.ch),
      '}' => token = Lexer::new_token(&TokenType::RBRACE, self.ch),
      _ => {
        if Lexer::is_letter(self.ch) {
          let lit = self.read_identifier();
          token = Token {
            token_type: lookup_ident(&lit),
            literal: lit,
          };
          return token;
        } else if Lexer::is_digit(self.ch) {
          let num = self.read_number();
          token = Token {
            token_type: &TokenType::INT,
            literal: num,
          };
          return token;
        } else {
          return token;
        }
      }
    }
    self.read_char();
    token
  }

  fn read_identifier(&mut self) -> String {
    let position = self.position;
    while Lexer::is_letter(self.ch) {
      self.read_char()
    }
    self.input[position..self.position].to_string()
  }

  fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
  }

  fn read_number(&mut self) -> String {
    let position = self.position;
    while Lexer::is_digit(self.ch) {
      self.read_char();
    }

    self.input[position..self.position].to_string()
  }

  fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
  }
}
