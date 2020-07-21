use super::*;
use actions::parse_actions;
use errors::{LocationError, TokenizeError};
use files::CodeLocation;
use statics::{valid_name_char, NameBuilder};
use types::parse_type;

#[derive(Debug, Clone)]
pub struct Function {
  pub location: CodeLocation,
  pub name: Option<String>,
  pub args: Vec<(String, Type)>,
  pub res: Option<Type>,
  pub body: Actions,
}

pub fn parse_function(t: &mut Tokenizer, anonymous: bool) -> Result<Function, LocationError> {
  let location = t.last_index_location();

  // Parse the function name
  let mut name_builder: Option<NameBuilder> = None;
  loop {
    match t.must_next_char()? {
      '\t' | '\n' | ' ' => {
        if let Some(_) = name_builder {
          // Not a valid name char return error
          return t.error(TokenizeError::InvalidNameChar);
        }
      }
      '(' => {
        // end of function name, start parsing arguments
        break;
      }
      c if valid_name_char(c) => {
        // Parsing the function name
        if let Some(name) = &mut name_builder {
          name.push(c);
        } else {
          name_builder = Some(NameBuilder::new_with_char(c));
        }
      }
      _ => {
        // Not a valid name char return error
        return t.error(TokenizeError::InvalidNameChar);
      }
    }
  }
  let name = if let Some(name) = name_builder {
    if anonymous {
      return t.error(TokenizeError::Custom("anonymous function with name"));
    }
    Some(name.to_string(t)?)
  } else {
    if !anonymous {
      return t.error(TokenizeError::Custom("function without name"));
    }
    None
  };

  // Parse the function args
  let mut args: Vec<(String, Type)> = vec![];
  'argsLoop: loop {
    let mut name = NameBuilder::new();
    loop {
      match t.must_next_char()? {
        '\t' | '\n' | ' ' => {
          if name.len() > 0 {
            break;
          }
        }
        ')' => {
          if name.len() > 0 {
            // Argument not completed
            return t.error(TokenizeError::IncompletedArgument);
          }

          // end of argument, start parsing response
          break 'argsLoop;
        }

        c if valid_name_char(c) => {
          // Parsing the function name
          name.push(c);
        }
        _ => {
          // Not a valid name char return error
          return t.error(TokenizeError::InvalidNameChar);
        }
      }
    }

    let type_ = parse_type(t, false)?;

    let break_after = match t.must_next_while_empty()? {
      ')' => true,
      ',' => false,
      _ => return t.error(TokenizeError::InvalidNameChar),
    };

    args.push((name.to_string(t)?, type_));

    if break_after {
      break;
    }
  }

  let mut res: Option<Type> = None;
  if t.must_next_while_empty()? != '{' {
    res = Some(parse_type(t, true)?);
    let c = t.must_next_while_empty()?;
    if c != '{' {
      return t.unexpected_char(c);
    }
  }

  let body = parse_actions(t)?;

  Ok(Function {
    location,
    name,
    args,
    res,
    body,
  })
}
