use super::*;
use action::{ActionToExpect, ParseAction};
use errors::LocationError;
use files::CodeLocation;
use statics::{valid_name_char, NameBuilder};
use types::parse_type;

#[derive(Debug, Clone)]
pub enum VarType {
  Let,
  Const,
}

#[derive(Debug, Clone)]
pub struct Variable {
  pub var_type: VarType,
  pub data_type: Option<Type>,
  pub name: String,
  pub action: Box<Action>,
  pub location: CodeLocation,
}

impl Into<ActionType> for Variable {
  fn into(self) -> ActionType {
    ActionType::Variable(self)
  }
}

pub fn parse_var<'a, 'b>(
  t: &'b mut Tokenizer<'a>,
  var_type_option: Option<VarType>,
) -> Result<Variable, LocationError> {
  let location = t.last_index_location();
  let mut name = NameBuilder::new();
  let mut data_type: Option<Type> = None;

  let var_type = if let Some(type_) = var_type_option {
    type_
  } else {
    let to_match = vec![&Keywords::Const, &Keywords::Let];
    let match_result = t.try_match(to_match);
    if let None = match_result {
      return t.unexpected_char(t.last_char());
    }

    if let Keywords::Const = match_result.unwrap() {
      VarType::Const
    } else {
      VarType::Let
    }
  };

  // Parse name
  let mut next_char = t.must_next_while_empty()?;
  loop {
    match next_char {
      c if valid_name_char(c) => name.push(c),
      ' ' | '\t' | '\n' => break,
      ':' | '=' => {
        t.index -= 1;
        break;
      }
      c => return t.unexpected_char(c),
    }
    next_char = t.must_next_char()?;
  }

  // Parse the variable type if set
  next_char = t.must_next_while_empty()?;
  if next_char == ':' {
    data_type = Some(parse_type(t, true)?);
    next_char = t.must_next_while_empty()?;
  }

  // Check for the = symbol
  match next_char {
    '=' => {}
    c => return t.unexpected_char(c),
  }

  // Parse the action after the action after the =
  let action = ParseAction::start(t, false, ActionToExpect::Assignment(""))?;

  Ok(Variable {
    location,
    var_type,
    data_type,
    name: name.to_string(t)?,
    action: Box::new(action),
  })
}
