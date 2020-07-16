pub mod action;
pub mod actions;
pub mod function;
pub mod globals;
pub mod numbers;
pub mod statics;
pub mod strings;
pub mod types;
pub mod utils;
pub mod variable;

use super::errors;
use super::files;
pub use action::{Action, ActionAssigment, ActionFor, ActionFunctionCall, ActionWhile};
pub use actions::Actions;
pub use function::Function;
pub use globals::{DataType, Tokenizer};
pub use numbers::Number;
pub use strings::String_;
pub use types::{Enum, GlobalType, Struct, Type};
pub use variable::{VarType, Variable};

#[cfg(test)]
mod tests;
