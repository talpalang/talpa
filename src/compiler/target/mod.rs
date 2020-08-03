mod builder;
mod golang;
mod javascript;

use super::*;
pub use anylize::AnilizedTokens;
pub use builder::{Block, BuildItems, Inline, LangBuilder};
use golang::Go;
use javascript::JavaScript;
pub use tokenize::{
  Action, ActionFor, ActionFunctionCall, ActionIf, ActionType, ActionWhile, Actions, Enum,
  Function, GlobalType, Number, NumberType, String_, Struct, Type, TypeType, VarType, Variable,
};

#[derive(Clone)]
pub enum Lang {
  // Currently dead because we do not yet have a working CLI with a JS option
  // TODO: fix this
  #[allow(dead_code)]
  JS,
  Go,
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
