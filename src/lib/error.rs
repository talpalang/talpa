use super::*;
use std::error::Error;

pub struct ParsingError {
  pub location: CodeLocation,
  pub error_type: ParsingErrorType,
  pub prev_line: Option<String>,
  pub line: String,
  pub next_line: Option<String>,
}

impl ParsingError {
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

impl Error for ParsingError {}

impl std::fmt::Debug for ParsingError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.err())
  }
}

impl Display for ParsingError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.err())
  }
}

#[derive(Debug)]
pub enum ParsingErrorType {
  IncompletedArgument,
  UnexpectedEOF,
  UnexpectedChar(char),
  UnexpectedResult,
  InvalidNameChar,
  Custom(&'static str),
}

impl Display for ParsingErrorType {
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

#[derive(Debug)]
pub enum LangErrorType {
  UnsupportedLang,
}

impl Display for LangErrorType {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::UnsupportedLang => write!(f, "Unsupported language"),
    }
  }
}
