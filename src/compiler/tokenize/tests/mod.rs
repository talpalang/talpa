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
use files::File;

fn new_file(contents: &str) -> File {
  let bytes = contents.as_bytes().to_vec();
  File::new(bytes, "test.tp")
}

// Parse a string of code
pub fn parse_str<'a>(contents: impl Into<String>) -> Tokenizer {
  let file = new_file(&contents.into());
  Tokenizer::tokenize(file).unwrap()
}

// Parse a string of code that is meant to fail
pub fn parse_str_fail(contents: impl Into<String>) {
  let res = Tokenizer::tokenize(new_file(&contents.into()));
  if let Ok(parsed_content) = res {
    panic!("{:?}", parsed_content);
  }
}
