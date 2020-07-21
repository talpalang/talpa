mod comments;
mod enums;
mod functions;
mod general;
mod global_types;
mod ifs;
mod loops;
mod structs;
mod variables;

use super::*;

// Parse a string of code
pub fn parse_str(contents: impl Into<String>) -> Tokenizer {
  Tokenizer::tokenize(DataType::Direct(contents.into().as_bytes().to_vec())).unwrap()
}

// Parse a string of code that is meant to fail
pub fn parse_str_fail(contents: impl Into<String>) {
  let res = Tokenizer::tokenize(DataType::Direct(contents.into().as_bytes().to_vec()));
  if let Ok(parsed_content) = res {
    panic!("{:?}", parsed_content);
  }
}
