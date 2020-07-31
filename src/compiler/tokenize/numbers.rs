use super::*;
use errors::{LocationError, TokenizeError};
use files::CodeLocation;

#[derive(Debug, Clone)]
pub struct Number {
  pub type_: NumberType,
  pub location: CodeLocation,
}

#[derive(Debug, Clone)]
pub enum NumberType {
  /// This matches the default int number type of the programming language,
  /// Note that the size of this value might differ over multiple languages
  Int(i64),

  /// This matches the default float number type of the programming language,
  /// Note that the size of this value might differ over multiple languages
  Float(f64),
}

pub enum NumberTypes {
  /// Detects the correct type automaticly
  Auto,
  // Int,
  // Float,
}

impl Into<ActionType> for Number {
  fn into(self) -> ActionType {
    ActionType::StaticNumber(self)
  }
}

pub struct NumberParser<'a, 'b> {
  t: &'b mut Tokenizer<'a>,
  buff: Vec<u8>,
  location: CodeLocation,
}

impl<'a, 'b> NumberParser<'a, 'b> {
  pub fn new_without_starting(t: &'b mut Tokenizer<'a>, buff: Vec<u8>) -> Self {
    let location = t.last_index_location();
    Self { t, buff, location }
  }
  pub fn result(&self, type_: NumberTypes) -> Result<Number, LocationError> {
    let type_ = match type_ {
      // NumberTypes::Float => Number::Float(self.to_float()?),
      // NumberTypes::Int => Number::Int(self.to_int()?),
      NumberTypes::Auto => {
        if self.buff.contains(&('.' as u8)) {
          NumberType::Float(self.to_float()?)
        } else {
          NumberType::Int(self.to_int()?)
        }
      }
    };
    Ok(Number {
      type_,
      location: self.location.clone(),
    })
  }
  fn to_float(&self) -> Result<f64, LocationError> {
    self.err(self.to_string()?.parse::<f64>())
  }
  fn to_int(&self) -> Result<i64, LocationError> {
    self.err(self.to_string()?.parse::<i64>())
  }
  fn err<T, E>(&self, err: Result<T, E>) -> Result<T, LocationError> {
    match err {
      Ok(v) => Ok(v),
      Err(_) => self.t.error(TokenizeError::Custom("Invalid number")),
    }
  }
  fn to_string(&self) -> Result<String, LocationError> {
    self.err(String::from_utf8(self.buff.clone()))
  }

  /*
    The parser is no where used so the code is commented out for now
  */

  // pub fn start(t: &'a mut Tokenizer) -> Result<Self, ParsingError> {
  //   let mut parser = Self { p, buff: vec![] };
  //   parser.parse()?;
  //   Ok(parser)
  // }
  // pub fn start_with_buffer(t: &'a mut Tokenizer, buff: Vec<u8>) -> Result<Self, ParsingError> {
  //   let mut parser = Self { p, buff };
  //   parser.parse()?;
  //   Ok(parser)
  // }
  // fn parse(&mut self) -> Result<(), ParsingError> {
  //   if self.buff.len() > 0 {
  //     if let None = self.t.next_while("\n\t ") {
  //       return self.t.unexpected_eof();
  //     };
  //     self.t.index -= 1;
  //   }

  //   while let Some(c) = self.t.next_char() {
  //     match c {
  //       '.' if !self.buff.contains(&('.' as u8)) => {}
  //       '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {}
  //       _ => break,
  //     }
  //     self.buff.push(c as u8);
  //   }

  //   self.t.index -= 1;
  //   Ok(())
  // }
}
