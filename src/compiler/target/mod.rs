mod builder;
mod javascript;
mod golang;

use super::*;
pub use anylize::AnilizedTokens;
pub use builder::{Block, BuildItems, Inline, LangBuilder};
use javascript::JavaScript;
use golang::Go;
pub use tokenize::{
  Action, ActionFor, ActionFunctionCall, ActionType, Actions, Enum, Function, GlobalType, Number,
  NumberType, String_, Struct, VarType, Variable,
};

#[derive(Clone)]
pub enum Lang {
  JS,
  Go
}

pub fn generate(t: AnilizedTokens, lang: Lang) -> Result<String, LocationError> {
  let mut lb = LangBuilder::new();
  let code = match lang {
    Lang::JS => JavaScript::generate(&mut lb, t),
    Lang::Go => Go::generate(&mut lb, t),
  };
  return match code {
    Ok(_) => Ok(format!("{}", lb)),
    Err(error) => Err(error),
  };
}
