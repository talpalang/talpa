use super::*;
use actions::ParseActions;
use errors::{LocationError, TokenizeError};
use statics::{valid_name_char, NameBuilder};
use types::parse_type;

#[derive(Debug)]
pub struct Function {
  pub name: Option<String>,
  pub args: Vec<(String, Type)>,
  pub body: Actions,
}

impl Function {
  fn empty() -> Self {
    Self {
      name: None,
      args: vec![],
      body: Actions::empty(),
    }
  }
}

#[derive(Debug)]
struct ParseFunctionStateNothing {
  function_name: Option<NameBuilder>,
}

#[derive(Debug)]
struct ParseFunctionStateArg {
  name: NameBuilder,
  type_: Option<Type>,
  parsing_name: bool,
}

impl ParseFunctionStateArg {
  fn new() -> Self {
    Self {
      name: NameBuilder::new(),
      type_: None,
      parsing_name: true,
    }
  }
}

#[derive(Debug)]
enum ParseFunctionState {
  Nothing(ParseFunctionStateNothing),
  Arg(ParseFunctionStateArg),
  AfterArg,
  Response,
}

pub struct ParseFunction<'a> {
  t: &'a mut Tokenizer,
  res: Function,
  state: ParseFunctionState,
}

impl<'a> ParseFunction<'a> {
  fn change_state(&mut self, to: ParseFunctionState) -> Result<(), LocationError> {
    // Check if the current state has data and if so commit it to the response
    match &self.state {
      ParseFunctionState::Nothing(info) => {
        if let Some(name) = &info.function_name {
          self.res.name = Some(name.to_string(self.t)?);
        }
      }
      ParseFunctionState::Arg(info) if !info.parsing_name && info.name.len() > 0 => {
        if let Some(type_) = &info.type_ {
          self
            .res
            .args
            .push((info.name.to_string(self.t)?, type_.clone()));
        }
      }
      ParseFunctionState::Arg(_) => {}
      ParseFunctionState::AfterArg => {}
      ParseFunctionState::Response => {}
    }

    self.state = to;
    Ok(())
  }
  pub fn start(t: &'a mut Tokenizer) -> Result<Function, LocationError> {
    let mut s = Self {
      t,
      res: Function::empty(),
      state: ParseFunctionState::Nothing(ParseFunctionStateNothing {
        function_name: None,
      }),
    };
    s.parse()?;
    Ok(s.res)
  }
  fn parse(&mut self) -> Result<(), LocationError> {
    while let Some(c) = self.t.next_char() {
      match &mut self.state {
        ParseFunctionState::Nothing(meta) => match c {
          '\t' | '\n' | ' ' => {
            if let Some(_) = meta.function_name {
              // Not a valid name char return error
              return self.t.error(TokenizeError::InvalidNameChar);
            }
          }
          '(' => {
            self.change_state(ParseFunctionState::Arg(ParseFunctionStateArg::new()))?;
            // end of function name, start parsing arguments
          }
          c if valid_name_char(c) => {
            // Parsing the function name
            if let Some(function_name) = &mut meta.function_name {
              function_name.push(c);
            } else {
              meta.function_name = Some(NameBuilder::new_with_char(c));
            }
          }
          _ => {
            // Not a valid name char return error
            return self.t.error(TokenizeError::InvalidNameChar);
          }
        },
        ParseFunctionState::Arg(meta) => match c {
          '\t' | '\n' | ' ' => {
            if meta.name.len() > 0 {
              meta.parsing_name = false;
            }
          }
          ')' => match meta.type_ {
            None if meta.name.len() > 0 => {
              // Argument not completed
              return self.t.error(TokenizeError::IncompletedArgument);
            }
            _ => {
              // End of argument
              self.change_state(ParseFunctionState::Response)?;
            }
          }, // end of argument, start parsing response
          // Parse the argument type
          _ if !meta.parsing_name => {
            meta.type_ = Some(parse_type(self.t, true)?);
            self.change_state(ParseFunctionState::AfterArg)?;
          }
          c if valid_name_char(c) => {
            // Parsing the function name
            meta.name.push(c);
          }
          _ => {
            // Not a valid name char return error
            return self.t.error(TokenizeError::InvalidNameChar);
          }
        },
        ParseFunctionState::AfterArg => match c {
          '\t' | '\n' | ' ' => {}
          ')' => {
            self.change_state(ParseFunctionState::Response)?;
          }
          ',' => {
            self.change_state(ParseFunctionState::Arg(ParseFunctionStateArg::new()))?;
          }
          _ => {
            // This is not what we are searching for
            return self.t.error(TokenizeError::InvalidNameChar);
          }
        },
        ParseFunctionState::Response => match c {
          '{' => {
            self.res.body = ParseActions::start(self.t)?;
            return Ok(());
          }
          _ => {}
        },
      }
    }
    Ok(())
  }
}
