use super::*;

#[derive(Debug)]
pub struct Actions {
  pub list: Vec<Action>,
}

impl Actions {
  pub fn empty() -> Self {
    Self { list: vec![] }
  }
}

enum ParseActionsState {
  Nothing,
}

pub struct ParseActions<'a> {
  p: &'a mut Parser,
  res: Actions,
  state: ParseActionsState,
}

impl<'a> ParseActions<'a> {
  pub fn start(p: &'a mut Parser) -> Result<Actions, LocationError> {
    let mut s = Self {
      p,
      res: Actions::empty(),
      state: ParseActionsState::Nothing,
    };
    s.parse()?;
    Ok(s.res)
  }
  fn parse(&mut self) -> Result<(), LocationError> {
    while let Some(c) = self.p.next_char() {
      match self.state {
        ParseActionsState::Nothing => match c {
          '\t' | '\n' | ' ' => {
            // Ignore these chars
          }
          '}' => return Ok(()),
          _ if valid_name_char(c) => {
            let action = ParseAction::start(self.p, true, ActionToExpect::ActionInBody)?;
            self.res.list.push(action);

            if let None = self.p.next_while("\n\t ") {
              return self.p.unexpected_eof();
            }
            self.p.index -= 1;
          }
          c => return self.p.unexpected_char(c),
        },
      }
    }
    Ok(())
  }
}
