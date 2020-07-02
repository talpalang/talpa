use super::*;

#[derive(Debug)]
pub struct String_ {
  pub content: String,
}

impl Into<Action> for String_ {
  fn into(self) -> Action {
    Action::StaticString(self)
  }
}

pub fn parse_static_str<'a>(p: &'a mut Parser) -> Result<String_, ParsingError> {
  let mut res = String_ {
    content: String::new(),
  };
  let mut string_content: Vec<u8> = vec![];

  let mut escaped = false;
  while let Some(c) = p.next_char() {
    match c {
      '\\' if !escaped => escaped = true,
      '"' if !escaped => {
        res.content = String::from_utf8(string_content).unwrap();
        return Ok(res);
      }
      _ => string_content.push(c as u8),
    }
    if escaped {
      escaped = false;
    }
  }

  p.unexpected_eof()
}
