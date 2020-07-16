mod javascript;

use super::*;
pub use anylize::AnilizedTokens;
use javascript::JavaScript;
pub use tokenize::{
  Action, ActionFor, ActionFunctionCall, Actions, Enum, Function, GlobalType, Number, String_,
  Struct, VarType, Variable,
};

pub enum Lang {
  JS,
}

pub fn generate(t: AnilizedTokens, lang: Lang) -> Result<String, LocationError> {
  let code = match lang {
    Lang::JS => JavaScript::generate(t),
  };
  return match code {
    Ok(res) => Ok(res.src),
    Err(error) => Err(error),
  };
}
