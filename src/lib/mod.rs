mod action;
mod actions;
mod error;
mod function;
pub mod languages;
mod numbers;
mod parser;
pub mod statics;
mod strings;
mod types;
mod variable;

pub use action::{Action, ActionToExpect, ParseAction, ParseActionState};
pub use actions::{Actions, ParseActions};
pub use error::{ParsingError, ParsingErrorType};
pub use function::{Function, ParseFunction};
pub use numbers::{Number, NumberParser, NumberTypes};
pub use parser::{CodeLocation, Parser};
pub use statics::{legal_name_char, Keywords, NameBuilder};
pub use std::fmt::Display;
pub use strings::{parse_static_str, String_};
pub use types::{ParseType, Type};
pub use variable::{parse_var, VarType, Variable};

#[cfg(test)]
mod tests;
