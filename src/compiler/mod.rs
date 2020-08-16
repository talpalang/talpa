pub mod anylize;
pub mod errors;
pub mod files;
pub mod target;
pub mod tokenize;

use anylize::anilize_tokens;
pub use anylize::AnilizedTokens;
pub use errors::LocationError;
use errors::TokenizeError;
pub use files::{CodeLocation, File, Path};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
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

pub enum Work {
  ParseFile(Path),
}

pub struct Compiler {
  opened_files: HashMap<String, Rc<Vec<u8>>>,
  options: Options,
  props: Rc<RefCell<dyn CompilerProps>>,
  work_todo: VecDeque<Work>,
}

impl Compiler {
  pub fn add_work(&mut self, work: Work) {
    self.work_todo.push_back(work);
  }

  fn open_file(&mut self, file_name: &str) -> Result<File, LocationError> {
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
      work_todo: VecDeque::from(vec![Work::ParseFile(Path::from(entry_file_name))]),
      options,
      props,
    };

    // TODO: remake this, we should probebly have one AnilizedTokens as output
    let mut first_res: Option<AnilizedTokens> = None;

    loop {
      let todo = match c.work_todo.pop_front() {
        None => break,
        Some(data) => data,
      };

      match todo {
        Work::ParseFile(to_parse_file_name) => {
          let entry_file = match c.open_file(&to_parse_file_name.to_string()) {
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
          let (formatted_res, anilize_res) = anilize_tokens(&mut c, res);

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

          first_res = Some(formatted_res);
        }
      }
    }

    if let Some(res) = first_res {
      if let Some(lang) = c.options.lang {
        let src = match generate(res, lang) {
          Err(err) => {
            c.props.borrow_mut().error(err);
            return;
          }
          Ok(v) => v,
        };

        if c.options.debug {
          c.props
            .borrow_mut()
            .debug_parsed_output(String::from("main.tp"), src) // TODO: fix the main.tp here
        }
      }
    }
  }
}
