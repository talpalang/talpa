mod javascript;

use super::*;
pub use javascript::JavaScript;

#[derive(Debug)]
pub struct LangError {
  language: String
}

pub enum Lang {
  JS
}

impl Into<&'static str> for Lang {
  fn into(self) -> &'static str {
    match self {
      Self::JS => "JavaScript"
    }
  }
}


pub fn generate(parser: Parser, lang: Lang) -> Result<String, String> {
  let code = match lang {
    Lang::JS => JavaScript::generate(parser)
  };
  match code {
    Ok(res) => return Ok(res.src),
    Err(err) => return Err(err.to_string())
  }
}
