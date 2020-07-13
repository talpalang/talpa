mod javascript;

use super::*;
pub use javascript::JavaScript;

pub enum Lang {
  JS,
}

pub fn generate(parser: Parser, lang: Lang) -> Result<String, CodeError> {
  let code = match lang {
    Lang::JS => JavaScript::generate(parser),
  };
  return match code {
    Ok(res) => Ok(res.src),
    Err(error) => Err(error),
  };
}
