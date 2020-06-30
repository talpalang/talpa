use super::*;

#[derive(Debug, Clone)]
pub struct Type {
  pub name: String,
}

impl Type {
  fn empty() -> Self {
    Self {
      name: String::new(),
    }
  }
}

struct ParseTypeStateTypeName {
  name: Vec<u8>,
}

impl ParseTypeStateTypeName {
  fn new() -> Self {
    Self { name: vec![] }
  }
}

enum ParseTypeState {
  TypeName(ParseTypeStateTypeName),
}

pub struct ParseType<'a> {
  p: &'a mut Parser,
  res: Type,
  state: ParseTypeState,
}

impl<'a> ParseType<'a> {
  pub fn start(p: &'a mut Parser, go_back_one: bool) -> Result<Type, ParsingError> {
    if go_back_one {
      p.index -= 1;
    }
    let mut s = Self {
      p,
      res: Type::empty(),
      state: ParseTypeState::TypeName(ParseTypeStateTypeName::new()),
    };
    s.parse()?;
    Ok(s.res)
  }
  fn parse(&mut self) -> Result<(), ParsingError> {
    while let Some(c) = self.p.next_char() {
      match &mut self.state {
        ParseTypeState::TypeName(meta) => match c {
          _ if legal_name_char(c) => {
            meta.name.push(c as u8);
          }
          _ => {
            self.p.index -= 1;
            self.res.name = String::from_utf8(meta.name.clone()).unwrap();
            return Ok(());
          }
        },
      }
    }
    Ok(())
  }
}
