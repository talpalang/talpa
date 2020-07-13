use super::*;
use core::fmt::Display;
use files::CodeLocation;
use std::error::Error;

#[derive(Clone)]
pub struct LocationError {
  pub location: CodeLocation,
  pub error_type: StateError,
  pub prev_line: Option<String>,
  pub line: Option<String>,
  pub next_line: Option<String>,
}

impl LocationError {
  fn err(&self) -> String {
    let mut output: Vec<String> = vec![];

    let err = self.clone();

    match (err.line, err.location.y, err.location.file_name) {
      (Some(line), Some(y), file_name) => {
        if let Some(file_name) = file_name {
          output.push(format!("Error in file: {}", file_name));
        } else {
          output.push("Error:".into());
        }

        let x = if let Some(x) = err.location.x { x } else { 0 };

        if let Some(line) = err.prev_line.clone() {
          output.push(format!("{}: {}", y - 1, line.replace("\t", "  ")));
        }

        let mut spacing = String::from("");
        for _ in 0..x + y.to_string().len() + format!("{}", y).len() + 1 {
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
      }
      (_, _, Some(file_name)) => {
        output.push(format!("Error in file: {}\n{}", file_name, err.error_type));
      }
      _ => {
        output.push(format!("Error:\n{}", err.error_type));
      }
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
  // Target(TargetError),
  IO(IOError),
}

impl Display for StateError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Tokenize(error) => write!(f, "{}", error),
      Self::IO(error) => write!(f, "IO error: {}", error),
      // Self::Target(error) => write!(f, "{}", error),
    }
  }
}

#[derive(Clone)]
pub enum TokenizeError {
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
