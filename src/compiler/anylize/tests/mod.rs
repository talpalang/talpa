mod comments;
mod enums;
mod functions;
mod general;
mod global_types;
mod ifs;
mod imports;
mod loops;
mod structs;
mod variables;

use super::*;
use std::fmt::Debug;
use tokenize::Tokenizer;

fn new_file(contents: &str) -> File {
  let bytes = contents.as_bytes().to_vec();
  File::new(bytes, "test.tp")
}

// Parse a string of code and validate it
pub fn parse_str(contents: impl Into<String>) -> AnilizedTokens {
  let res = Tokenizer::tokenize(new_file(&contents.into())).unwrap();
  let (tokens, anilize_res) = anilize_tokens(res);
  if anilize_res.errors.len() > 0 {
    panic!("{:?}", anilize_res.errors);
  }
  tokens
}

// Parse a string of code and expext it somewhere to fail
pub fn parse_str_fail(contents: impl Into<String>) {
  let res = match Tokenizer::tokenize(new_file(&contents.into())) {
    Ok(res) => res,
    Err(_) => return, // Test is successfull
  };

  let (new_res, anilize_res) = anilize_tokens(res);
  if anilize_res.errors.len() == 0 {
    panic!("{:?}", new_res);
  }
}

// Parse a string of code and expext it somewhere to fail
pub fn parse_str_fail_with_meta(contents: impl Into<String>, meta: impl Debug) {
  let res = match Tokenizer::tokenize(new_file(&contents.into())) {
    Ok(res) => res,
    Err(_) => return, // Test is successfull
  };

  let (new_res, anilize_res) = anilize_tokens(res);
  if anilize_res.errors.len() == 0 {
    panic!("Res: {:?}\nMeta: {:?}", new_res, meta);
  }
}

// Parse a string of code and expext a warning
pub fn parse_str_warning(contents: impl Into<String>) {
  let res = Tokenizer::tokenize(new_file(&contents.into())).unwrap();

  let (new_res, anilize_res) = anilize_tokens(res);
  if anilize_res.warnings.len() == 0 {
    panic!("{:?}", new_res);
  }
}
