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
  Action, ActionFor, ActionFunctionCall, Actions, Function, Number, String_, VarType, Variable,
};

/// This contains compiler options, like the amound of threads to use or the target language
pub struct Options {
  pub lang: Lang,
}

pub struct Compiler {
  options: Options,
}

impl Compiler {
  pub fn start(options: Options) -> Result<(), CodeError> {
    let c = Self { options };

    let mut file = File::open("./example.tp").unwrap();
    let mut contents: Vec<u8> = vec![];
    file.read_to_end(&mut contents).unwrap();

    let res = Parser::parse(contents)?;
    println!("{:#?}", res);

    let code = generate(res, c.options.lang);
    let src = match code {
      Err(err) => {
        // TODO:
        // Make it so generate returns a CodeError so we can return it here instaid of printing it
        println!("{:?}", err);
        std::process::exit(1);
      }
      Ok(res) => res,
    };
    println!("{}", src);
    Ok(())
  }
}
