mod utils;

use super::*;
use tokenize::{DataType, Tokenizer};

// Parse a string of code and validate it
pub fn parse_str(contents: impl Into<String>) {
  let res = Tokenizer::tokenize(DataType::Direct(contents.into().as_bytes().to_vec())).unwrap();
  let (_, anilize_res) = anilize_tokens(&res);
  if anilize_res.errors.len() > 0 {
    panic!(anilize_res.errors);
  }
}

// Parse a string of code and expext it somewhere to fail
pub fn parse_str_fail(contents: impl Into<String>) {
  let res = match Tokenizer::tokenize(DataType::Direct(contents.into().as_bytes().to_vec())) {
    Ok(res) => res,
    Err(_) => return, // Test is successfull
  };

  let (_, anilize_res) = anilize_tokens(&res);
  if anilize_res.errors.len() == 0 {
    panic!("{:?}", res);
  }
}
