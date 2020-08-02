mod utils;

use super::*;
use tokenize::Tokenizer;

fn new_file(contents: &str) -> File {
  let bytes = contents.as_bytes().to_vec();
  File::new(bytes, "test.tp")
}

// Parse a string of code and validate it
pub fn parse_str(contents: impl Into<String>) {
  let res = Tokenizer::tokenize(new_file(&contents.into())).unwrap();
  let (_, anilize_res) = anilize_tokens(res);
  if anilize_res.errors.len() > 0 {
    panic!(anilize_res.errors);
  }
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
