mod errors;
pub mod files;
mod target;
pub mod tokenize;

use crate::compiler::target::generate;
pub use errors::*;
use std::fs::File;
use std::io::prelude::*;
pub use target::Lang;
use tokenize::Parser;
pub use tokenize::{
  Action, ActionFor, ActionFunctionCall, Actions, DataType, Function, Number, String_, VarType,
  Variable,
};

/// This contains compiler options, like the amound of threads to use or the target language
pub struct Options {
  pub lang: Lang,
}

pub struct Compiler {
  options: Options,
}

impl Compiler {
  pub fn start(options: Options) -> Result<(), LocationError> {
    let c = Self { options };

    let res = Parser::parse(DataType::File("./example.tp"))?;
    println!("Debug output:");
    println!("{:#?}", res);

    let src = generate(res, c.options.lang)?;
    println!("Parse output:");
    println!("{}", src);
    Ok(())
  }
}
