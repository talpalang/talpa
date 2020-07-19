use super::*;
use action::{ActionToExpect, ParseAction};
use errors::LocationError;
use statics::valid_name_char;

#[derive(Debug, Clone)]
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
  t: &'a mut Tokenizer,
  res: Actions,
  state: ParseActionsState,
}

impl<'a> ParseActions<'a> {
  pub fn start(t: &'a mut Tokenizer) -> Result<Actions, LocationError> {
    let mut s = Self {
      t,
      res: Actions::empty(),
      state: ParseActionsState::Nothing,
    };
    s.parse()?;
    Ok(s.res)
  }
  fn parse(&mut self) -> Result<(), LocationError> {
    while let Some(c) = self.t.next_char() {
      match self.state {
        ParseActionsState::Nothing => match c {
          '\t' | '\n' | ' ' => {
            // Ignore these chars
          }
          '}' => return Ok(()),
          _ if valid_name_char(c) => {
            let action = ParseAction::start(self.t, true, ActionToExpect::ActionInBody)?;
            self.res.list.push(action);

            self.t.must_next_while("\n\t ")?;
            self.t.index -= 1;
          }
          c => return self.t.unexpected_char(c),
        },
      }
    }
    Ok(())
  }
}
