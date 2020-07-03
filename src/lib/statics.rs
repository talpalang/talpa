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
  pub fn to_string<'a>(&self, p: &'a Parser) -> Result<String, ParsingError> {
    match String::from_utf8(self.0.clone()) {
      Ok(parsed_string) => match parsed_string.get(0..1) {
        Some(v) if "1234567890".contains(v) => {
          p.error(ParsingErrorType::Custom("name cannot start with a number"))
        }
        _ => Ok(parsed_string),
      },
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
