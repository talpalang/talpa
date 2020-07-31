use super::*;
use anylize::{AnylizeError, AnylizeWarning};
use core::fmt::Display;
use std::error::Error;

#[derive(Clone)]
pub struct LocationError {
  pub error_type: StateError,
  pub prev_line: Option<String>,
  pub line: Option<(String, usize, u16)>,
  pub next_line: Option<String>,
  pub file_name: String,
}

impl LocationError {
  pub fn only_file_name(error: impl Into<StateError>, file_name: String) -> Self {
    Self {
      error_type: error.into(),
      prev_line: None,
      line: None,
      next_line: None,
      file_name,
    }
  }
  fn err(&self) -> String {
    let mut output: Vec<String> = vec![];

    let err = self.clone();

    if let Some((line, x, y)) = err.line {
      output.push(format!("Error in file: {}", self.file_name));

      if let Some(line) = err.prev_line.clone() {
        output.push(format!("{}: {}", y - 1, line.replace("\t", "  ")));
      }

      let mut spacing = String::from("");
      for _ in 0..x + y.to_string().len() + 1 {
        spacing += " ";
      }
      output.push(format!(
        "{}: {}\n{}^-- {}",
        y,
        line.replace("\t", "  "),
        spacing,
        err.error_type,
      ));

      if let Some(line) = err.next_line.clone() {
        output.push(format!("{}: {}", y + 1, line.replace("\t", "  ")));
      }
    } else {
      output.push(format!(
        "Error in file: {}\n{}",
        self.file_name, err.error_type
      ));
    }

    format!("{}", output.join("\n"))
  }
}

impl Error for LocationError {}

impl std::fmt::Debug for LocationError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.err())
  }
}

impl Display for LocationError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.err())
  }
}

#[derive(Clone)]
pub enum StateError {
  Tokenize(TokenizeError),
  AnylizeError(AnylizeError),
  AnylizeWarning(AnylizeWarning),
  IO(IOError),
  // Target(TargetError),
}

impl Into<StateError> for AnylizeError {
  fn into(self) -> StateError {
    StateError::AnylizeError(self)
  }
}

impl Into<StateError> for AnylizeWarning {
  fn into(self) -> StateError {
    StateError::AnylizeWarning(self)
  }
}

impl Display for StateError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Tokenize(error) => write!(f, "{}", error),
      Self::AnylizeError(error) => write!(f, "{}", error),
      Self::AnylizeWarning(error) => write!(f, "{}", error),
      Self::IO(error) => write!(f, "IO error: {}", error),
      // Self::Target(error) => write!(f, "{}", error),
    }
  }
}

#[derive(Clone)]
pub enum TokenizeError {
  UnableToOpenFile(String),
  IncompletedArgument,
  UnexpectedEOF,
  UnexpectedChar(char),
  UnexpectedResult,
  InvalidNameChar,
  Custom(&'static str),
}

impl Into<StateError> for TokenizeError {
  fn into(self) -> StateError {
    StateError::Tokenize(self)
  }
}

impl Display for TokenizeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::UnableToOpenFile(file_name) => write!(f, "Unable to open file {}", file_name),
      Self::IncompletedArgument => write!(f, "Incompletted argument"),
      Self::UnexpectedEOF => write!(f, "Unexpected EOF"),
      Self::UnexpectedChar(c) => write!(f, "Unexpected char: {}", c),
      Self::UnexpectedResult => write!(f, "Unexpected result"),
      Self::InvalidNameChar => write!(f, "Invalid name char"),
      Self::Custom(error) => write!(f, "{}", error),
    }
  }
}

#[derive(Clone)]
pub enum IOError {
  IO(String),
}

impl Into<StateError> for IOError {
  fn into(self) -> StateError {
    StateError::IO(self)
  }
}

impl Display for IOError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::IO(error) => write!(f, "{}", error),
    }
  }
}

/*

Commented out for now as there are not yet any target errors

*/

// #[derive(Clone)]
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
