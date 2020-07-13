mod javascript;

use super::*;
pub use javascript::JavaScript;

pub enum Lang {
  JS,
}

pub fn generate(parser: Parser, lang: Lang) -> Result<String, TargetError> {
  let code = match lang {
    Lang::JS => JavaScript::generate(parser),
  };
  match code {
    Ok(res) => return Ok(res.src),
    Err(error) => return Err(error),
  }
}
