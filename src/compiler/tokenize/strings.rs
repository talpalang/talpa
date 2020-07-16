use super::*;
use errors::LocationError;

#[derive(Debug, Clone)]
pub struct String_ {
  pub content: String,
}

impl Into<Action> for String_ {
  fn into(self) -> Action {
    Action::StaticString(self)
  }
}

pub fn parse_static_str<'a>(t: &'a mut Tokenizer) -> Result<String_, LocationError> {
  let mut res = String_ {
    content: String::new(),
  };
  let mut string_content: Vec<u8> = vec![];

  let mut escaped = false;
  while let Some(c) = t.next_char() {
    match c {
      '\\' if !escaped => escaped = true,
      '"' if !escaped => {
        res.content = String::from_utf8(string_content).unwrap();
        return Ok(res);
      }
      _ => {
        string_content.push(c as u8);
        if escaped {
          escaped = false;
        }
      }
    }
  }

  t.unexpected_eof()
}
