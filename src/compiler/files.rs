use super::errors::{LocationError, StateError};
use std::rc::Rc;

/// This is used in meany places to safe the location of code
///
/// Why the index and y?
/// There are a few things to consider when storing code:
/// 1. How memory efficent is it because of all tokens you will need a locaton to later report errors?
/// 2. How much memory / cpu power does it cost to know these locations?
/// 3. How easially can we get from this to debug information with code?
///
/// 1. This solution is only 64 + 16 bits per location so pretty memory efficient
///
/// 2. Pretty low the compiler only has to track y and index this way so every char iteration only
/// cost 1 variable to update unless it's a newline in that case 2. This is pretty memory and cpu efficient
///
/// 3. Because we already know the line where location was created (y) we don't have to count every \n till the index.
/// Becuase of that we only have to seek 1 line backwards and forwards to get a view of the 3 lines an location was made on
#[derive(Debug, Clone)]
pub struct CodeLocation {
  pub index: usize,
  pub y: u16,
}

impl CodeLocation {
  pub fn new(index: usize, y: u16) -> Self {
    Self { index, y }
  }
}

#[derive(Clone, Debug)]
pub struct File {
  pub bytes: Rc<Vec<u8>>,
  pub name: String,
}

impl File {
  pub fn new(bytes: Vec<u8>, file_name: impl Into<String>) -> Self {
    let mut mut_bytes = bytes;

    let mut chars_to_remove: Vec<usize> = vec![];

    // Remove all the '\r' from the code because we currently do not support it
    for (i, c) in mut_bytes.iter().enumerate().rev() {
      if *c as char == '\r' {
        chars_to_remove.push(i);
      }
    }
    for i in chars_to_remove {
      mut_bytes.remove(i);
    }

    Self {
      bytes: Rc::new(mut_bytes),
      name: file_name.into(),
    }
  }

  pub fn must_error(&self, error: impl Into<StateError>, location: CodeLocation) -> LocationError {
    let mut index = location.index;
    let mut scan_prev_line = false;
    let mut prev_line_bytes: Vec<u8> = vec![];
    let mut line_bytes: Vec<u8> = vec![];
    let mut next_line_bytes: Vec<u8> = vec![];
    let mut x = 0;

    if index > 0 {
      loop {
        index -= 1;
        if let Some(c) = self.bytes.get(index) {
          match *c as char {
            '\n' => {
              scan_prev_line = true;
              break;
            }
            _ => line_bytes.insert(0, *c),
          };
          x += 1;
        }

        if index == 0 {
          break;
        }
      }

      if scan_prev_line {
        while let Some(c) = self.bytes.get(index) {
          if index == 0 {
            break;
          }
          index -= 1;
          match *c as char {
            '\n' => {
              break;
            }
            _ => prev_line_bytes.insert(0, *c),
          }
        }
      }
    }

    index = location.index;
    while let Some(c) = self.bytes.get(index) {
      index += 1;
      match *c as char {
        '\n' => break,
        _ => line_bytes.push(*c),
      }
    }

    while let Some(c) = self.bytes.get(index) {
      index += 1;
      match *c as char {
        '\n' => break,
        _ => next_line_bytes.push(*c),
      }
    }

    let prev_line = if scan_prev_line {
      Some(String::from_utf8(prev_line_bytes).unwrap())
    } else {
      None
    };

    let line = Some((String::from_utf8(line_bytes).unwrap(), x, location.y));

    let next_line = if next_line_bytes.len() > 0 {
      Some(String::from_utf8(next_line_bytes).unwrap())
    } else {
      None
    };

    LocationError {
      error_type: error.into(),
      prev_line,
      line,
      next_line,
      file_name: self.name.to_string(),
    }
  }

  pub fn error<T>(
    &self,
    error: impl Into<StateError>,
    location: CodeLocation,
  ) -> Result<T, LocationError> {
    Err(self.must_error(error, location))
  }
}
