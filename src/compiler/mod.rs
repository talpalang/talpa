pub mod anylize;
pub mod errors;
pub mod files;
pub mod target;
pub mod tokenize;

use anylize::anilize_tokens;
pub use anylize::AnilizedTokens;
pub use errors::LocationError;
pub use files::CodeLocation;
use target::generate;
pub use target::Lang;
use tokenize::{DataType, Tokenizer};

/// This contains compiler options, like the amound of threads to use or the target language
#[derive(Clone)]
pub struct Options {
  pub lang: Option<Lang>,
  pub debug: bool,
}

pub trait CompilerProps {
  /// The compiler will asks compiler options via this function
  fn get_options(&self) -> Options {
    Options {
      lang: None,
      debug: false,
    }
  }
  /// When a warning showsup this function will be called
  /// Note that this function might be called multiple times
  fn warning(&mut self, _: LocationError) {}
  /// When an error showsup this function will be called
  /// Note that this function might be called multiple times
  fn error(&mut self, _: LocationError) {}

  /// Once the tokens of a file have been anylized they will be send here
  /// Note: Options.debug must be enabled
  fn debug_formatted_tokens(&mut self, _: CodeLocation, _: AnilizedTokens) {}
  /// Once output is generated this function will be called
  /// Note: Options.debug must be enabled
  fn debug_parsed_output(&mut self, _: CodeLocation, _: String) {}
}

pub struct Compiler {
  options: Options,
}

impl Compiler {
  pub fn start<'a>(props: &'a mut impl CompilerProps) {
    let c = Self {
      options: props.get_options(),
    };

    let file_name = "./example.tp";
    let res = match Tokenizer::tokenize(DataType::File(file_name)) {
      Err(err) => {
        props.error(err);
        return;
      }
      Ok(v) => v,
    };
    let (formatted_res, anilize_res) = anilize_tokens(&res);

    // We don't need the res data anymore from here on wasted memory.
    drop(res);

    for warning in anilize_res.warnings {
      props.warning(LocationError::new_simple(warning));
    }

    if anilize_res.errors.len() > 0 {
      for error in anilize_res.errors {
        props.error(LocationError::new_simple(error));
      }
      return;
    }

    if c.options.debug {
      props.debug_formatted_tokens(
        CodeLocation::only_file_name(Some(file_name.into())),
        formatted_res.clone(),
      );
    }

    if let Some(lang) = c.options.lang {
      let src = match generate(formatted_res, lang) {
        Err(err) => {
          props.error(err);
          return;
        }
        Ok(v) => v,
      };

      if c.options.debug {
        props.debug_parsed_output(CodeLocation::empty(), src)
      }
    }
  }
}
