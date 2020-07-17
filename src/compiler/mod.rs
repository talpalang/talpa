pub mod anylize;
pub mod errors;
pub mod files;
pub mod target;
pub mod tokenize;

use anylize::anilize_tokens;
pub use errors::LocationError;
use target::generate;
pub use target::Lang;
use tokenize::{DataType, Tokenizer};

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

    let res = Tokenizer::tokenize(DataType::File("./example.tp"))?;
    let (formatted_res, anilize_res) = anilize_tokens(&res);

    // We don't need the res data anymore from here on wasted memory.
    drop(res);

    for error in anilize_res.errors {
      return Err(LocationError::new_simple(error));
    }

    println!("Debug output:");
    println!("{:#?}", formatted_res);

    let src = generate(formatted_res, c.options.lang)?;
    println!("Parse output:");
    println!("{}", src);
    Ok(())
  }
}
