mod javascript;

use super::*;
pub use javascript::JavaScript;

pub enum Lang {
  JS
}

impl Into<Lang> for String {
  fn into(self) -> Lang {
    let _js = String::from("js");
    match self {
      _js => Lang::JS
    }
  }
}

pub fn generate(parser: Parser, lang: String) -> Result<String, ParsingErrorType> {
  let code = match lang.into() {
    Lang::JS => JavaScript::generate(parser)
  };
  match code {
    Ok(res) => return Ok(res.src),
    Err(err) => return Err(ParsingErrorType::LangError)
  }
}
