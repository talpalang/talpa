use super::*;

#[derive(Debug)]
pub enum VarType {
  Let,
  Const,
}

#[derive(Debug)]
pub struct Variable {
  pub var_type: VarType,
  pub data_type: Option<Type>,
  pub name: String,
  pub action: Box<Action>,
}

impl Into<Action> for Variable {
  fn into(self) -> Action {
    Action::Variable(self)
  }
}

pub fn parse_var<'a>(
  p: &'a mut Parser,
  var_type_option: Option<VarType>,
) -> Result<Variable, ParsingError> {
  let mut name = NameBuilder::new();
  let mut data_type: Option<Type> = None;

  let var_type = if let Some(type_) = var_type_option {
    type_
  } else {
    let to_match = vec![&Keywords::Const, &Keywords::Let];
    let match_result = p.try_match(to_match);
    if let None = match_result {
      return p.unexpected_char(*p.contents.get(p.index).unwrap() as char);
    }

    if let Keywords::Const = match_result.unwrap() {
      VarType::Const
    } else {
      VarType::Let
    }
  };

  // Parse name
  let mut next_char = p.next_while(" \t\n");
  loop {
    if let Some(c) = next_char {
      match c {
        _ if valid_name_char(c) => name.push(c),
        ' ' | '\t' | '\n' => break,
        ':' | '=' => {
          p.index -= 1;
          break;
        }
        c => return p.unexpected_char(c),
      }
    } else {
      return p.unexpected_eof();
    }
    next_char = p.next_char();
  }

  // Parse the variable type if set
  next_char = p.next_while(" \t\n");
  if let None = next_char {
    return p.unexpected_eof();
  }
  if next_char.unwrap() == ':' {
    data_type = Some(parse_type(p, true)?);
    next_char = p.next_while(" \t\n");
  }

  // Check for the = symbol
  match next_char {
    Some('=') => {}
    Some(c) => return p.unexpected_char(c),
    None => return p.unexpected_eof(),
  }

  // Parse the action after the action after the =
  let action = ParseAction::start(p, false, ActionToExpect::Assignment(""))?;

  Ok(Variable {
    var_type,
    data_type,
    name: name.to_string(p)?,
    action: Box::new(action),
  })
}
