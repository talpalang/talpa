mod javascript;

use super::*;
pub use javascript::JavaScript;

pub enum Lang {
  JS,
  Unknown,
}

impl Into<Lang> for String {
  fn into(self) -> Lang {
    match &self[..] {
      "js" => Lang::JS,
      _ => Lang::Unknown,
    }
  }
}

impl<'a> Into<Lang> for &'a str {
  fn into(self) -> Lang {
    match self {
      "js" => Lang::JS,
      _ => Lang::Unknown,
    }
  }
}

pub fn generate(parser: Parser, lang: impl Into<Lang>) -> Result<String, LangErrorType> {
  let code = match lang.into() {
    Lang::JS => JavaScript::generate(parser),
    Lang::Unknown => return Err(LangErrorType::UnsupportedLang),
  };
  match code {
    Ok(res) => return Ok(res.src),
    Err(error) => return Err(error),
  }
}
