use super::*;
use errors::{LocationError, StateError, TokenizeError};
use files::{CodeLocation, File};
use function::parse_function;
use import::parse_import;
use std::collections::HashMap;
use std::fmt;
use types::{parse_enum, parse_global_type, parse_struct};
use utils::MatchString;
use variable::parse_var;

pub struct Tokenizer {
  pub file: File,
  pub index: usize,
  pub y: u16,
  pub functions: Vec<Function>,
  pub vars: Vec<Variable>,
  pub structs: Vec<Struct>,
  pub enums: Vec<Enum>,
  pub types: Vec<GlobalType>,
  pub imports: Vec<Import>,
}

#[derive(Debug)]
struct SimpleTokenizer<'a> {
  pub functions: &'a Vec<Function>,
  pub vars: &'a Vec<Variable>,
  pub structs: &'a Vec<Struct>,
  pub enums: &'a Vec<Enum>,
  pub types: &'a Vec<GlobalType>,
  pub imports: &'a Vec<Import>,
}

impl<'a> fmt::Debug for Tokenizer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let simple_tokenized = SimpleTokenizer {
      functions: &self.functions,
      vars: &self.vars,
      structs: &self.structs,
      enums: &self.enums,
      types: &self.types,
      imports: &self.imports,
    };
    writeln!(f, "{:#?}", simple_tokenized)
  }
}

impl Tokenizer {
  pub fn tokenize(file: File) -> Result<Self, LocationError> {
    let mut tokenizer = Self {
      index: 0,
      y: 1,
      file,
      functions: vec![],
      vars: vec![],
      structs: vec![],
      enums: vec![],
      types: vec![],
      imports: vec![],
    };

    tokenizer.parse_nothing()?;
    Ok(tokenizer)
  }

  pub fn error<T>(&self, error: impl Into<StateError>) -> Result<T, LocationError> {
    self
      .file
      .error(error, CodeLocation::new(self.index, self.y))
  }

  pub fn unexpected_char<T>(&self, c: char) -> Result<T, LocationError> {
    self.error(TokenizeError::UnexpectedChar(c))
  }

  pub fn unexpected_eof<T>(&self) -> Result<T, LocationError> {
    self.error(TokenizeError::UnexpectedEOF)
  }

  pub fn last_char(&self) -> char {
    let letter = self
      .file
      .bytes
      .get(if self.index == 0 { 0 } else { self.index - 1 });

    if let Some(c) = letter {
      *c as char
    } else {
      0 as char
    }
  }

  pub fn must_next_char(&mut self) -> Result<char, LocationError> {
    if let Some(c) = self.next_char().2 {
      Ok(c)
    } else {
      self.unexpected_eof()
    }
  }

  /// Returns (added to index, added to y, char)
  pub fn next_char(&mut self) -> (usize, u16, Option<char>) {
    let letter = if let Some(l) = self.file.bytes.get(self.index) {
      *l as char
    } else {
      return (0, 0, None);
    };

    let start_index = self.index;
    let start_y = self.y;

    self.index += 1;
    if letter == '\n' {
      self.y += 1;
    }

    // check for the start of a comment
    if letter != '/' {
      return (self.index - start_index, self.y - start_y, Some(letter));
    }

    // check for next forward slash
    match self.file.bytes.get(self.index) {
      Some(b'/') => {
        // detected single line comment
        loop {
          let next = if let Some(l) = self.file.bytes.get(self.index) {
            *l as char
          } else {
            return (self.index - start_index, self.y - start_y, None);
          };

          self.index += 1;

          // check for newline (end of comment)
          if next == '\n' {
            self.y += 1;

            return (
              self.index - start_index,
              self.y - start_y,
              self.next_char().2,
            );
          }
        }
      }
      Some(b'*') => {
        // detected multi-line comment
        loop {
          match self.file.bytes.get(self.index) {
            Some(b'\n') => {
              self.y += 1;
              self.index += 1;
            }
            Some(b'*') => {
              self.index += 1;

              // * detected
              if let Some(b'/') = self.file.bytes.get(self.index) {
                // */ detected
                self.index += 1;

                return (
                  self.index - start_index,
                  self.y - start_y,
                  self.next_char().2,
                );
              }
            }
            Some(_) => {
              self.index += 1;
            }
            None => return (self.index - start_index, self.y - start_y, None),
          }
        }
      }
      _ => return (self.index - start_index, self.y - start_y, Some(letter)),
    }
  }

  fn seek_next_char(&self) -> Option<char> {
    let letter = self.file.bytes.get(self.index)?;
    Some(*letter as char)
  }

  pub fn must_next_while(&mut self, chars: &'static str) -> Result<char, LocationError> {
    if let Some(c) = self.next_while(chars) {
      Ok(c)
    } else {
      self.unexpected_eof()
    }
  }

  pub fn must_next_while_empty(&mut self) -> Result<char, LocationError> {
    if let Some(c) = self.next_while(" \t\n") {
      Ok(c)
    } else {
      self.unexpected_eof()
    }
  }

  pub fn next_while(&mut self, chars: &'static str) -> Option<char> {
    while let Some(c) = self.next_char().2 {
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

    let start_index = self.index;
    let start_y = self.y;

    let mut char_count: usize = 0;
    let mut next_char = self.next_char();

    loop {
      if let Some(c) = next_char.2 {
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

        next_char = self.next_char();
        if new_options_vec.len() == 0 {
          break;
        }
        options_vec = new_options_vec;
        char_count += 1;
      } else {
        self.index = start_index;
        self.y = start_y;
        return None;
      }
    }

    // Reset the index if we havent found the requested item
    self.index = start_index;
    self.y = start_y;

    None
  }

  fn parse_nothing(&mut self) -> Result<(), LocationError> {
    loop {
      if let Some(_) = self.next_while(" \n\t") {
        self.index -= 1;
        match self.try_match(vec![
          &Keywords::Const,
          &Keywords::Fn,
          &Keywords::Struct,
          &Keywords::Enum,
          &Keywords::Type,
          &Keywords::Import,
        ]) {
          Some(Keywords::Import) => {
            let mut parsed_import = parse_import(self)?;
            self.imports.append(&mut parsed_import);
          }
          Some(Keywords::Const) => {
            let parsed_variable = parse_var(self, Some(VarType::Const))?;
            self.vars.push(parsed_variable);
          }
          Some(Keywords::Fn) => {
            let parsed_function = parse_function(self, false)?;
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
            let c = self.last_char();
            if c != 0 as char {
              return self.unexpected_char(c);
            } else {
              return self.unexpected_eof();
            }
          }
        }
      } else {
        return Ok(());
      }
    }
  }

  pub fn expect(&mut self, text: &str) -> Result<(), LocationError> {
    for letter in text.chars() {
      match self.next_char().2 {
        Some(v) if v == letter => {}
        Some(c) => return self.unexpected_char(c),
        None => return self.unexpected_eof(),
      }
    }
    Ok(())
  }

  /// This return the last location
  /// return values: index, y
  pub fn last_index(&self) -> (usize, u16) {
    if self.index == 0 {
      return (0, 0);
    }

    (
      self.index - 1,
      if let Some('\n') = self.seek_next_char() {
        self.y - 1
      } else {
        self.y
      },
    )
  }

  pub fn last_index_location(&self) -> CodeLocation {
    let (index, y) = self.last_index();
    CodeLocation::new(index, y)
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
