use super::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Parser {
  pub index: usize,
  pub contents: Vec<u8>,
  pub functions: Vec<Function>,
}

impl Parser {
  pub fn error<T>(&self, error_type: ParsingErrorType) -> Result<T, ParsingError> {
    self.custom_error(error_type, None)
  }
  pub fn unexpected_char<T>(&self, c: char) -> Result<T, ParsingError> {
    self.error(ParsingErrorType::UnexpectedChar(c))
  }
  pub fn unexpected_eof<T>(&self) -> Result<T, ParsingError> {
    self.error(ParsingErrorType::UnexpectedEOF)
  }
  pub fn custom_error<T>(
    &self,
    error_type: ParsingErrorType,
    file_char_number: Option<usize>,
  ) -> Result<T, ParsingError> {
    let use_index = if let Some(index) = file_char_number {
      index
    } else {
      self.index - 1
    };
    let mut line_number = 1;
    let mut current_line_position = 1;
    let mut prev_line_bytes: Option<Vec<u8>> = None;
    let mut current_line = vec![];

    for (index, letter) in self.contents.iter().enumerate() {
      if index == use_index {
        break;
      }
      match *letter as char {
        '\n' => {
          prev_line_bytes = Some(current_line);
          current_line = vec![];
          line_number += 1;
          current_line_position = 0;
        }
        '\r' => {} // Ignore this char
        letter_char => {
          current_line.push(*letter);
          current_line_position += if letter_char == '\t' { 2 } else { 1 };
        }
      }
    }

    let mut prev_line = None;
    if let Some(line_data) = prev_line_bytes {
      prev_line = Some(String::from_utf8(line_data).unwrap())
    }

    let mut next_line_bytes: Option<Vec<u8>> = None;
    let iterrator = self.contents.iter().skip(use_index);
    for letter in iterrator {
      match *letter as char {
        '\n' => {
          if let Some(_) = next_line_bytes {
            break;
          }
          next_line_bytes = Some(vec![]);
        }
        '\r' => {} // Ignore this char
        _ => {
          if let Some(mut line) = next_line_bytes {
            line.push(*letter);
            next_line_bytes = Some(line);
          } else {
            current_line.push(*letter);
          }
        }
      }
    }

    let next_line = if let Some(bytes) = next_line_bytes {
      Some(String::from_utf8(bytes).unwrap())
    } else {
      None
    };

    let res = ParsingError {
      location: CodeLocation {
        file_name: None,
        y: line_number,
        x: current_line_position,
      },
      error_type,
      prev_line,
      line: String::from_utf8(current_line).unwrap(),
      next_line: next_line,
    };
    Err(res)
  }
  pub fn parse(contents: impl Into<Vec<u8>>) -> Result<Self, ParsingError> {
    let mut parser = Self {
      index: 0,
      contents: contents.into(),
      functions: vec![],
    };
    parser.parse_nothing()?;
    Ok(parser)
  }
  pub fn next_char(&mut self) -> Option<char> {
    // get next
    let letter = *self.contents.get(self.index)? as char;

    // define forward slash, newline & astrix
    let fs = '/' as u8;
    let nl = '\n' as u8;
    let ast = '*' as u8;

    // increase index
    self.index += 1;

    // check for forward slash
    if letter == &fs {
      // check for next forward slash
      let next = self.contents.get(self.index)?;
      if next == &fs {
        // detected single line comment
        // loop until newline (comments are not parsed)
        loop {
          let next = self.contents.get(self.index)?;
          self.index += 1;
          // check for end line
          if next == &nl {
            // new line detected (end of comment)
            return self.next_char();
          }
        }
      } else if letter == &ast {
        // detected multi-line comment
        // loop until closed (comments are not parsed)
        loop {
          let next = self.contents.get(self.index)?;
          self.index += 1;
          if next == &ast {
            // * detected
            let last = self.contents.get(self.index)?;
            if last == &ast {
              // */ detected
              self.index += 1;
              break
            }
          }
        }
      }
    }
    Some(*letter as char)
  }
  fn seek_next_char(&mut self) -> Option<char> {
    let letter = self.contents.get(self.index)?;
    Some(*letter as char)
  }
  pub fn next_while(&mut self, chars: &'static str) -> Option<char> {
    while let Some(c) = self.next_char() {
      if !chars.contains(c) {
        return Some(c);
      }
    }
    None
  }

  /// Tries to match something
  /// The second string for the options array is for checking if the matched value has a certen surfix
  /// The next char after the matched value will be checked against it
  /// For example surfix "abc" will match the following matched string surfix: 'a', 'b' or 'c'
  pub fn try_match<'a, T>(&mut self, options: &[(T, &'static str)]) -> Option<T>
  where
    T: Into<&'a str> + Copy,
  {
    if options.len() == 0 {
      return None;
    }

    let mut surfix_map: HashMap<&'a str, &'static str> = HashMap::with_capacity(options.len());
    let mut options_vec: Vec<&str> = vec![];
    for option in options.iter() {
      if option.0.into().len() == 0 {
        continue;
      }
      options_vec.push(&option.0.into());

      if option.1.len() > 0 {
        surfix_map.insert(option.0.into(), option.1);
      }
    }

    let mut char_count: usize = 0;
    while let Some(c) = self.next_char() {
      let mut new_options_vec: Vec<&str> = vec![];
      for option in options_vec {
        if option.len() <= char_count {
          continue;
        }
        match option.as_bytes().get(char_count) {
          Some(found_char) if *found_char as char == c => {
            if option.len() != char_count + 1 {
              new_options_vec.push(&option);
              continue;
            }

            if let Some(must_match_surfix) = surfix_map.get(option) {
              // This option contains a surfix match, lets test it here
              let next_char = self.seek_next_char();
              if let None = next_char {
                continue;
              } else if !must_match_surfix.contains(next_char.unwrap()) {
                continue;
              }
            }

            for opt in options {
              if opt.0.into() == option {
                return Some(opt.0);
              }
            }
            return None;
          }
          _ => continue,
        }
      }
      if new_options_vec.len() == 0 {
        break;
      }
      options_vec = new_options_vec;
      char_count += 1;
    }

    // Reset the index if we havent found the requested item
    self.index -= char_count + 1;
    None
  }
  fn expect_next(&mut self, c: char) -> Result<(), ParsingError> {
    match self.next_char() {
      Some(v) if v == c => Ok(()),
      Some(c) => self.error(ParsingErrorType::UnexpectedChar(c)),
      None => self.error(ParsingErrorType::UnexpectedEOF),
    }
  }
  fn parse_nothing(&mut self) -> Result<(), ParsingError> {
    while let Some(c) = self.next_char() {
      match c {
        'f' => {
          self.expect_next('n')?;
          let new_func = ParseFunction::start(self)?;
          self.functions.push(new_func);
        }
        _ => {}
      };
    }
    Ok(())
  }

  /*
      Functions written but not used so commented out
  */

  // fn expect(&mut self, text: &str) -> Result<(), ParsingError> {
  //     for letter in text.chars() {
  //         match self.next_char() {
  //             Some(v) if v == letter => {}
  //             Some(_) => return self.error(ParsingErrorType::UnexpectedChar, None),
  //             None => {
  //                 return self.error(ParsingErrorType::UnexpectedEOF, None);
  //             }
  //         }
  //     }
  //     Ok(())
  // }

  // fn forward_until(
  //     &mut self,
  //     allowed_chars: impl Into<String>,
  //     until: char,
  // ) -> Result<(), ParsingError> {
  //     let allowed_chars_string = allowed_chars.into();
  //     while let Some(c) = self.next_char() {
  //         if c == until {
  //             return Ok(());
  //         }
  //         if !allowed_chars_string.contains(c) {
  //             return self.error(ParsingErrorType::UnexpectedChar);
  //         }
  //     }
  //     self.error(ParsingErrorType::UnexpectedEOF)
  // }
}

#[derive(Debug)]
pub struct CodeLocation {
  pub file_name: Option<String>,
  pub x: usize,
  pub y: usize,
}
