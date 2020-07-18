mod builder;
mod javascript;

use super::*;
pub use anylize::AnilizedTokens;
pub use builder::{Block, BuildItems, Inline, LangBuilder};
use javascript::JavaScript;
pub use tokenize::{
  Action, ActionFor, ActionFunctionCall, Actions, Enum, Function, GlobalType, Number, String_,
  Struct, VarType, Variable,
};

pub enum Lang {
  JS,
}

pub fn generate(t: AnilizedTokens, lang: Lang) -> Result<String, LocationError> {
  let mut lb = LangBuilder::new();
  let code = match lang {
    Lang::JS => JavaScript::generate(&mut lb, t),
  };
  return match code {
    Ok(_) => Ok(format!("{}", lb)),
    Err(error) => Err(error),
  };
}
