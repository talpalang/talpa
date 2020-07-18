use super::*;
use errors::{IOError, LocationError, StateError, TokenizeError};
use files::CodeLocation;
use function::ParseFunction;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use types::{parse_enum, parse_global_type, parse_struct};
use utils::MatchString;
use variable::parse_var;

pub struct Tokenizer {
  contents: Vec<u8>,
  file_name: Option<String>,
  pub index: usize,
  pub functions: Vec<Function>,
  pub vars: Vec<Variable>,
  pub structs: Vec<Struct>,
  pub enums: Vec<Enum>,
  pub types: Vec<GlobalType>,
}

#[derive(Debug)]
struct SimpleTokenizer<'a> {
  pub file_name: &'a Option<String>,
  pub functions: &'a Vec<Function>,
  pub vars: &'a Vec<Variable>,
  pub structs: &'a Vec<Struct>,
  pub enums: &'a Vec<Enum>,
  pub types: &'a Vec<GlobalType>,
}

impl fmt::Debug for Tokenizer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let simple_tokenized = SimpleTokenizer {
      file_name: &self.file_name,
      functions: &self.functions,
      vars: &self.vars,
      structs: &self.structs,
      enums: &self.enums,
      types: &self.types,
    };
    writeln!(f, "{:#?}", simple_tokenized)
  }
}

pub enum DataType<'a> {
  /// Use a file as input to start reading from
  File(&'a str),

  /// Parse directly from bytes
  /// Currently only used for testing so we allow it to be dead code for now
  ///
  /// TODO: Let this not be dead code :)
  #[allow(dead_code)]
  Direct(Vec<u8>),
}

impl Tokenizer {
  pub fn tokenize(contents: DataType) -> Result<Self, LocationError> {
    let mut tokenizer = Self {
      index: 0,
      contents: vec![],
      functions: vec![],
      vars: vec![],
      structs: vec![],
      enums: vec![],
      types: vec![],
      file_name: None,
    };

    let mut tokens = match contents {
      DataType::File(location) => {
        let mut file = match File::open(location) {
          Ok(f) => f,
          Err(err) => return tokenizer.custom_error(IOError::IO(format!("{}", err)), None, true),
        };
        let mut contents: Vec<u8> = vec![];
        file.read_to_end(&mut contents).unwrap();
        contents
      }
      DataType::Direct(bytes) => bytes,
    };

    let mut chars_to_remove: Vec<usize> = vec![];

    // Remove all the '\r' from the code because we currently do not support it
    for (i, c) in tokens.iter().enumerate().rev() {
      if *c as char == '\r' {
        chars_to_remove.push(i);
      }
    }
    for i in chars_to_remove {
      tokens.remove(i);
    }

    tokenizer.contents = tokens;

    tokenizer.parse_nothing()?;
    Ok(tokenizer)
  }

  pub fn error<T, Y>(&self, error: Y) -> Result<T, LocationError>
  where
    Y: Into<StateError>,
  {
    self.custom_error(error, None, false)
  }

  pub fn unexpected_char<T>(&self, c: char) -> Result<T, LocationError> {
    self.error(TokenizeError::UnexpectedChar(c))
  }

  pub fn unexpected_eof<T>(&self) -> Result<T, LocationError> {
    self.error(TokenizeError::UnexpectedEOF)
  }

  pub fn custom_error<T, Y>(
    &self,
    error: Y,
    file_char_number: Option<usize>,
    only_file_name: bool,
  ) -> Result<T, LocationError>
  where
    Y: Into<StateError>,
  {
    if only_file_name {
      return Err(LocationError {
        location: CodeLocation::only_file_name(self.file_name.clone()),
        error_type: error.into(),
        prev_line: None,
        line: None,
        next_line: None,
      });
    }

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

    Err(LocationError {
      location: CodeLocation {
        file_name: self.file_name.clone(),
        y: Some(line_number),
        x: Some(current_line_position),
      },
      error_type: error.into(),
      prev_line,
      line: Some(String::from_utf8(current_line).unwrap()),
      next_line: next_line,
    })
  }

  pub fn last_char<'a>(&'a self) -> char {
    if self.index == 0 {
      // There aren't any chars read yet return 0
      // This is not really anywhere used so we can better return null than to return an option
      // Just overcomplicates things when it issn't needed
      return 0 as char;
    }
    return *self.contents.get(self.index - 1).unwrap() as char;
  }

  pub fn next_char(&mut self) -> Option<char> {
    let letter = *self.contents.get(self.index)? as char;
    self.index += 1;

    // check for the start of a comment
    if letter != '/' {
      return Some(letter);
    }

    // check for next forward slash
    match *self.contents.get(self.index)? as char {
      '/' => {
        // detected single line comment
        loop {
          let next = *self.contents.get(self.index)? as char;
          self.index += 1;
          // check for newline (end of comment)
          if next == '\n' {
            return self.next_char();
          }
        }
      }
      '*' => {
        // detected multi-line comment
        loop {
          let next = *self.contents.get(self.index)? as char;
          self.index += 1;
          if next == '*' {
            // * detected
            let last = *self.contents.get(self.index)? as char;
            if last == '/' {
              // */ detected
              self.index += 1;
              return self.next_char();
            }
          }
        }
      }
      _ => return Some(letter),
    }
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
  pub fn try_match<'a, T>(&mut self, options: Vec<&'a T>) -> Option<&'a T>
  where
    T: MatchString,
  {
    if options.len() == 0 {
      return None;
    }

    let mut meta_map: HashMap<&'static str, &'a T> = HashMap::with_capacity(options.len());
    let mut options_vec: Vec<&str> = vec![];

    for option in options {
      let option_str = option.get_string();
      if option_str.len() == 0 {
        continue;
      }
      options_vec.push(option_str);
      meta_map.insert(option_str, option);
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

            match meta_map.get(option) {
              Some(meta) => {
                if let Some(next_char_needs_to_match) = meta.after() {
                  // This option contains a surfix match, test test it here
                  let next_char = self.seek_next_char();
                  if let None = next_char {
                    continue;
                  } else if !next_char_needs_to_match.contains(next_char.unwrap()) {
                    continue;
                  }
                }

                return Some(meta)
              }
              None => panic!("A critical error has occured, please create an issue at https://github.com/talpalang/talpa/issues with your code so we can resolve this"),
            }
          }
          _ => {}
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

  fn parse_nothing(&mut self) -> Result<(), LocationError> {
    if let None = self.next_while(" \n\t") {
      return Ok(());
    }
    self.index -= 1;
    while let Some(_) = self.next_while(" \n\t") {
      self.index -= 1;
      match self.try_match(vec![
        &Keywords::Fn,
        &Keywords::Const,
        &Keywords::Struct,
        &Keywords::Enum,
        &Keywords::Type,
      ]) {
        Some(Keywords::Const) => {
          let parsed_variable = parse_var(self, Some(VarType::Const))?;
          self.vars.push(parsed_variable);
        }
        Some(Keywords::Fn) => {
          let parsed_function = ParseFunction::start(self)?;
          self.functions.push(parsed_function);
        }
        Some(Keywords::Struct) => {
          let parsed_struct = parse_struct(self, false, false)?;
          self.structs.push(parsed_struct);
        }
        Some(Keywords::Enum) => {
          let parsed_enum = parse_enum(self, false, false)?;
          self.enums.push(parsed_enum);
        }
        Some(Keywords::Type) => {
          let parsed_type = parse_global_type(self)?;
          self.types.push(parsed_type);
        }
        _ => {
          // could be newline/tab/whitespace
          return if let Some(c) = self.next_char() {
            self.unexpected_char(c)
          } else {
            self.unexpected_eof()
          };
        }
      }
    }
    Ok(())
  }

  pub fn expect(&mut self, text: &str) -> Result<(), LocationError> {
    for letter in text.chars() {
      match self.next_char() {
        Some(v) if v == letter => {}
        Some(c) => return self.unexpected_char(c),
        None => return self.unexpected_eof(),
      }
    }
    Ok(())
  }

  pub fn get_file_name(&self) -> Option<String> {
    self.file_name.clone()
  }

  /*
      Functions written but not used so commented out
  */

  // pub fn match_name(&mut self) -> Result<(String, usize), ParsingError> {
  //   let mut name = NameBuilder::new();

  //   while let Some(c) = self.next_char() {
  //     match c {
  //       ' ' | '\t' | '\n' if name.len() == 0 => {} // Ignore this char
  //       _ if legal_name_char(c) => name.push(c),
  //       _ => break,
  //     }
  //   }

  //   self.index -= 1;
  //   let name_len = name.len();
  //   let res_name = name.to_string(self)?;
  //   Ok((res_name, name_len))
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
