use super::*;
use action::{ActionToExpect, ParseAction};
use errors::LocationError;
use statics::valid_name_char;

#[derive(Debug, Clone)]
pub struct Actions {
  pub actions: Vec<Action>,
}

impl Actions {
  pub fn empty() -> Self {
    Self { actions: vec![] }
  }
}

pub fn parse_actions(t: &mut Tokenizer) -> Result<Actions, LocationError> {
  let mut res = Actions::empty();

  loop {
    let c = t.must_next_while_empty()?;
    if c == '}' {
      return Ok(res);
    }
    if !valid_name_char(c) {
      return t.unexpected_char(c);
    }

    let action = ParseAction::start(t, true, ActionToExpect::ActionInBody)?;
    res.actions.push(action);
  }
}
