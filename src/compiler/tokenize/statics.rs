use super::*;
use errors::{LocationError, TokenizeError};
use numbers::NumberParser;
use utils::MatchString;

pub static VALID_NAME_CHARS: &'static str =
  "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_";

pub fn valid_name_char(c: char) -> bool {
  VALID_NAME_CHARS.contains(c)
}

#[derive(Debug)]
pub struct NameBuilder(Vec<u8>);

impl NameBuilder {
  pub fn new() -> Self {
    Self(vec![])
  }
  /// new_with_char creates a new name builder with a start char.
  /// you will need to check first_char if it's a valid name char
  pub fn new_with_char(first_char: char) -> Self {
    Self(vec![first_char as u8])
  }
  pub fn is_boolean(&self) -> Option<Boolean> {
    if self.0.clone() == b"true" {
      Some(Boolean(true))
    } else if self.0.clone() == b"false" {
      Some(Boolean(false))
    } else {
      None
    }
  }
  pub fn is_number<'a, 'b>(&self, t: &'a mut Tokenizer) -> Option<NumberParser<'a>> {
    for letter in &self.0 {
      match *letter as char {
        '.' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {}
        _ => return None,
      }
    }
    let parser = NumberParser::new_without_starting(t, self.0.clone());
    Some(parser)
  }
  pub fn to_string<'a>(&self, t: &'a Tokenizer) -> Result<String, LocationError> {
    if self.len() == 0 {
      return Ok(String::new());
    }
    if let Some(c) = self.0.get(0) {
      match *c as char {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
          return t.error(TokenizeError::Custom("name cannot start with a number"))
        }
        _ => {}
      }
    }

    match String::from_utf8(self.0.clone()) {
      Ok(parsed_string) => Ok(parsed_string),
      Err(_) => t.error(TokenizeError::Custom("Invalid utf8 string")),
    }
  }
  pub fn len(&self) -> usize {
    self.0.len()
  }
  pub fn push(&mut self, value: char) {
    self.0.push(value as u8);
  }
}

#[derive(Clone, Copy)]
pub enum Keywords {
  Fn,
  If,
  Let,
  Pub,
  For,
  Loop,
  Else,
  Enum,
  Type,
  True,
  False,
  Const,
  While,
  Break,
  Return,
  Struct,
  Import,
  Continue,
}

impl Keywords {
  pub fn is_keyword(word: &str) -> bool {
    let lower_word = word.to_lowercase();
    let words = [
      "fn", "let", "for", "loop", "type", "enum", "const", "while", "break", "struct", "return",
      "continue", "if", "else", "true", "false", "import", "pub",
    ];
    words.contains(&lower_word.as_str())
  }
}

impl MatchString for Keywords {
  fn get_string(&self) -> &'static str {
    match self {
      Self::If => "if",
      Self::Fn => "fn",
      Self::Let => "let",
      Self::For => "for",
      Self::Pub => "pub",
      Self::Loop => "loop",
      Self::Else => "else",
      Self::Type => "type",
      Self::Enum => "enum",
      Self::True => "true",
      Self::False => "false",
      Self::Const => "const",
      Self::While => "while",
      Self::Break => "break",
      Self::Struct => "struct",
      Self::Import => "import",
      Self::Return => "return",
      Self::Continue => "continue",
    }
  }
  fn after(&self) -> Option<&'static str> {
    Some(" \t\n")
  }
}

impl<'a> From<&'a Keywords> for &'static str {
  fn from(keywords: &'a Keywords) -> &'static str {
    keywords.into()
  }
}
