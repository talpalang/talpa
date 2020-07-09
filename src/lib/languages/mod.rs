use super::*;
mod javascript;
pub use javascript::JavaScript;

#[derive(Debug)]
pub struct LangError {
  language: String
}

pub fn generate(parser: Parser, lang: &str) -> Result<String, String> {
  let code;
  match lang {
    "javascript" => code = JavaScript::generate(parser),
    &_ => return Err(String::from("Language not supported"))
  }
  match code {
    Ok(res) => return Ok(res.src),
    Err(err) => return Err(err.to_string())
  }
}
