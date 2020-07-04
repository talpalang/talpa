use super::*;

pub static VALID_NAME_CHARS: &'static str =
  "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_";

#[derive(Debug)]
pub struct NameBuilder(Vec<u8>);

impl NameBuilder {
  pub fn new() -> Self {
    Self(vec![])
  }
  pub fn new_with_char(first_char: char) -> Self {
    Self(vec![first_char as u8])
  }
  pub fn is_number<'a>(&self, p: &'a mut Parser) -> Option<NumberParser<'a>> {
    for letter in &self.0 {
      match *letter as char {
        '.' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {}
        _ => return None,
      }
    }
    let parser = NumberParser::new_without_starting(p, self.0.clone());
    Some(parser)
  }
  pub fn to_string<'a>(&self, p: &'a Parser) -> Result<String, ParsingError> {
    if self.len() == 0 {
      return Ok(String::new());
    }
    if let Some(c) = self.0.get(0) {
      match *c as char {
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
          return p.error(ParsingErrorType::Custom("name cannot start with a number"))
        }
        _ => {}
      }
    }

    match String::from_utf8(self.0.clone()) {
      Ok(parsed_string) => Ok(parsed_string),
      Err(_) => p.error(ParsingErrorType::Custom("Invalid utf8 string")),
    }
  }
  pub fn len(&self) -> usize {
    self.0.len()
  }
  pub fn push(&mut self, value: char) {
    self.0.push(value as u8);
  }
}

pub fn legal_name_char(c: char) -> bool {
  VALID_NAME_CHARS.contains(c)
}

#[derive(Clone, Copy)]
pub enum Keywords {
  Const,
  Let,
  Return,
  Fn,
}

impl Into<&'static str> for Keywords {
  fn into(self) -> &'static str {
    match self {
      Self::Const => "const",
      Self::Let => "let",
      Self::Return => "return",
      Self::Fn => "fn",
    }
  }
}
