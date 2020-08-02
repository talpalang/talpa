pub mod anylize;
pub mod errors;
pub mod files;
pub mod target;
pub mod tokenize;

use anylize::anilize_tokens;
pub use anylize::AnilizedTokens;
pub use errors::LocationError;
use errors::TokenizeError;
pub use files::{CodeLocation, File};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use target::generate;
pub use target::Lang;
use tokenize::Tokenizer;

/// This contains compiler options, like the amound of threads to use or the target language
#[derive(Clone)]
pub struct Options {
  pub lang: Option<Lang>,
  pub debug: bool,
}

pub trait CompilerProps {
  /// This requests to open a file
  fn open_file(&mut self, file_name: &str) -> Result<Vec<u8>, String>;

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
  fn debug_formatted_tokens(&mut self, _: String, _: AnilizedTokens) {}
  /// Once output is generated this function will be called
  /// Note: Options.debug must be enabled
  fn debug_parsed_output(&mut self, _: String, _: String) {}
}

pub struct Compiler {
  opened_files: HashMap<String, Rc<Vec<u8>>>,
  options: Options,
  props: Rc<RefCell<dyn CompilerProps>>,
}

impl Compiler {
  pub fn open_file(&mut self, file_name: &str) -> Result<File, LocationError> {
    if let Some(bytes) = self.opened_files.get(file_name) {
      Ok(File {
        bytes: Rc::clone(bytes),
        name: file_name.to_string(),
      })
    } else {
      let bytes = match self.props.borrow_mut().open_file(file_name) {
        Err(_) => {
          return Err(LocationError::only_file_name(
            TokenizeError::UnableToOpenFile(file_name.to_string()),
            file_name.to_string(),
          ))
        }
        Ok(v) => v,
      };

      let file = File::new(bytes, file_name);
      self
        .opened_files
        .insert(file_name.to_string(), Rc::clone(&file.bytes));
      Ok(file)
    }
  }

  pub fn start<'a>(entry_file_name: &str, props: Rc<RefCell<dyn CompilerProps>>) {
    let options = {
      let props = props.borrow_mut();
      props.get_options()
    };

    let mut c = Self {
      opened_files: HashMap::new(),
      options,
      props,
    };

    let entry_file = match c.open_file(entry_file_name) {
      Ok(val) => val,
      Err(err) => {
        c.props.borrow_mut().error(err);
        return;
      }
    };

    let res = match Tokenizer::tokenize(entry_file) {
      Err(err) => {
        c.props.borrow_mut().error(err);
        return;
      }
      Ok(v) => v,
    };

    let file_name = res.file.name.clone();
    let (formatted_res, anilize_res) = anilize_tokens(res);

    for warning in anilize_res.warnings {
      c.props.borrow_mut().warning(warning);
    }

    if anilize_res.errors.len() > 0 {
      for error in anilize_res.errors {
        c.props.borrow_mut().error(error);
      }
      return;
    }

    if c.options.debug {
      c.props
        .borrow_mut()
        .debug_formatted_tokens(file_name.clone(), formatted_res.clone());
    }

    if let Some(lang) = c.options.lang {
      let src = match generate(formatted_res, lang) {
        Err(err) => {
          c.props.borrow_mut().error(err);
          return;
        }
        Ok(v) => v,
      };

      if c.options.debug {
        c.props
          .borrow_mut()
          .debug_parsed_output(file_name.clone().to_string(), src)
      }
    }
  }
}
