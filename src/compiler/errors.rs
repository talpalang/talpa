use super::*;
use core::fmt::Display;
use files::CodeLocation;
use std::error::Error;

pub struct CodeError {
  pub location: CodeLocation,
  pub error_type: StateError,
  pub prev_line: Option<String>,
  pub line: String,
  pub next_line: Option<String>,
}

impl CodeError {
  fn err(&self) -> String {
    let mut output: Vec<String> = vec![];
    let y = self.location.y;

    if let Some(line) = self.prev_line.clone() {
      output.push(format!("{}: {}", y - 1, line.replace("\t", "  ")));
    }

    let mut spacing = String::from("");
    for _ in 0..self.location.x + y.to_string().len() + format!("{}", y).len() + 1 {
      spacing += " ";
    }
    output.push(format!(
      "{}: {}\n{}^-- {}",
      y,
      self.line.replace("\t", "  "),
      spacing,
      self.error_type,
    ));

    if let Some(line) = self.next_line.clone() {
      output.push(format!("{}: {}", y + 1, line.replace("\t", "  ")));
    }

    format!("{}", output.join("\n"))
  }
}

impl Error for CodeError {}

impl std::fmt::Debug for CodeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.err())
  }
}

impl Display for CodeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.err())
  }
}

pub enum StateError {
  Tokenize(TokenizeError),
  // Target(TargetError),
}

impl Display for StateError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Tokenize(error) => write!(f, "{}", error),
      // Self::Target(error) => write!(f, "{}", error),
    }
  }
}

#[derive(Debug)]
pub enum TokenizeError {
  IncompletedArgument,
  UnexpectedEOF,
  UnexpectedChar(char),
  UnexpectedResult,
  InvalidNameChar,
  Custom(&'static str),
}

impl Display for TokenizeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::IncompletedArgument => write!(f, "Incompletted argument"),
      Self::UnexpectedEOF => write!(f, "Unexpected EOF"),
      Self::UnexpectedChar(c) => write!(f, "Unexpected char: {}", c),
      Self::UnexpectedResult => write!(f, "Unexpected result"),
      Self::InvalidNameChar => write!(f, "Invalid name char"),
      Self::Custom(error) => write!(f, "{}", error),
    }
  }
}

/*

Commented out for now as there are not yet any target errors

*/

// #[derive(Debug)]
// pub enum TargetError {
//   UnsupportedLang,
// }

// impl Display for TargetError {
//   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//     match self {
//       Self::UnsupportedLang => write!(f, "Unsupported language"),
//     }
//   }
// }
