pub mod anylize;
pub mod errors;
pub mod files;
pub mod target;
pub mod tokenize;

use anylize::anilize_tokens;
pub use anylize::AnilizedTokens;
pub use errors::LocationError;
pub use files::CodeLocation;
use std::collections::HashMap;
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
  /// This requests to open a file
  fn open_file(&mut self, file_name: String) -> Result<Vec<u8>, String>;

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

pub struct Compiler<'a> {
  opened_files: HashMap<String, Vec<u8>>,
  options: Options,
  props: &'a mut (dyn CompilerProps + 'a),
}

impl<'a> Compiler<'a> {
  pub fn open_file(&mut self, file_name: String) -> Vec<u8> {
    // TODO return error here instaid of unwrapping
    self.props.open_file(file_name).unwrap()
  }

  pub fn start(entry: impl Into<String>, props: &'a mut impl CompilerProps) {
    let mut c = Self {
      opened_files: HashMap::new(),
      options: props.get_options(),
      props: &mut *props,
    };

    let res = match Tokenizer::tokenize(c.props.open_file(entry.into())) {
      Err(err) => {
        c.props.error(err);
        return;
      }
      Ok(v) => v,
    };
    let (formatted_res, anilize_res) = anilize_tokens(&res);

    // We don't need the res data anymore from here on wasted memory.
    drop(res);

    for warning in anilize_res.warnings {
      c.props.warning(LocationError::new_simple(warning));
    }

    if anilize_res.errors.len() > 0 {
      for error in anilize_res.errors {
        c.props.error(LocationError::new_simple(error));
      }
      return;
    }

    if c.options.debug {
      c.props.debug_formatted_tokens(
        CodeLocation::only_file_name(file_name.into()),
        formatted_res.clone(),
      );
    }

    if let Some(lang) = c.options.lang {
      let src = match generate(formatted_res, lang) {
        Err(err) => {
          c.props.error(err);
          return;
        }
        Ok(v) => v,
      };

      if c.options.debug {
        c.props.debug_parsed_output(CodeLocation::empty(), src)
      }
    }
  }
}
