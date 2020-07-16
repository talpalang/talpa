mod comments;
mod enums;
mod functions;
mod general;
mod global_types;
mod loops;
mod structs;
mod variables;

use super::*;
use globals::{DataType, Tokenizer};

// Parse a string of code
pub fn parse_str(contents: impl Into<String>) -> Tokenizer {
  Tokenizer::tokenize(DataType::Direct(contents.into().as_bytes().to_vec())).unwrap()
}

// Parse a string of code that is meant to fail
pub fn parse_str_fail(contents: impl Into<String>) {
  // Parse the code
  let res = Tokenizer::tokenize(DataType::Direct(contents.into().as_bytes().to_vec()));
  // If the code parsed without error
  if let Ok(parsed_content) = res {
    // There is a problem with the parser
    // Output the result in an error (failing the test)
    panic!("{:?}", parsed_content);
  }
}
