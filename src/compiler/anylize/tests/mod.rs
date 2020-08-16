mod comments;
mod enums;
mod functions;
mod general;
mod global_types;
mod ifs;
mod imports;
mod loops;
mod structs;
mod variables;

use super::*;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

/// parse multiple files and check if the output doesn't contain any errors
pub fn parse_files(contents: HashMap<String, String>) {
  let res = compile(contents);
  if res.borrow().errors.len() > 0 {
    panic!("{:?}", res.borrow().errors);
  }
}

/// Parse a string of code and validate it
pub fn parse_str<'a>(contents: impl Into<String>) -> AnilizedTokens {
  let res = single_file_compile(contents.into());
  if res.borrow().errors.len() > 0 {
    panic!("{:?}", res.borrow().errors);
  }
  let borrowed_res = res.borrow();
  borrowed_res.tokens.get("main.tp").unwrap().clone()
}

/// Parse a string of code and expext it somewhere to fail
pub fn parse_str_fail(contents: impl Into<String>) {
  parse_str_fail_meta(contents, None);
}

/// Parse a string of code and expext it somewhere to fail
pub fn parse_str_fail_with_meta(contents: impl Into<String>, meta: impl Into<String>) {
  parse_str_fail_meta(contents, Some(meta.into()));
}

fn parse_str_fail_meta(contents: impl Into<String>, meta_option: Option<String>) {
  let res = single_file_compile(contents.into());
  if res.borrow().errors.len() == 0 {
    let debug_string = if let Some(tokens) = res.borrow().tokens.get("main.tp") {
      format!("{:?}", tokens)
    } else {
      format!("{:?}", res)
    };

    if let Some(meta) = meta_option {
      panic!("Res: {}\nMeta: {}", debug_string, meta);
    }
    panic!("{}", debug_string);
  }
}

/// Parse a string of code and expext a warning
pub fn parse_str_warning(contents: impl Into<String>) {
  let res = single_file_compile(contents.into());
  if res.borrow().warnings.len() == 0 {
    if let Some(tokens) = res.borrow().tokens.get("main.tp") {
      panic!("{:?}", tokens);
    }
    panic!("{:?}", res);
  }
}

fn single_file_compile<'a>(data: String) -> Rc<RefCell<CompilerMeta>> {
  let mut files = HashMap::new();
  files.insert(String::from("main.tp"), data);
  compile(files)
}

fn compile<'a>(files: HashMap<String, String>) -> Rc<RefCell<CompilerMeta>> {
  let meta = Rc::new(RefCell::new(CompilerMeta {
    files,
    errors: vec![],
    warnings: vec![],
    tokens: HashMap::new(),
  }));

  let meta_clone = Rc::clone(&meta);
  Compiler::start("main.tp", meta);

  meta_clone
}

#[derive(Debug, Clone)]
struct CompilerMeta {
  files: HashMap<String, String>,
  errors: Vec<LocationError>,
  warnings: Vec<LocationError>,
  tokens: HashMap<String, AnilizedTokens>,
}

impl CompilerProps for CompilerMeta {
  fn open_file(&mut self, file_name: &str) -> Result<Vec<u8>, String> {
    if let Some(data) = self.files.get(file_name) {
      Ok(data.as_bytes().to_vec())
    } else {
      Err(String::from("File not found"))
    }
  }
  fn get_options(&self) -> Options {
    Options {
      lang: None,
      debug: true,
    }
  }
  fn warning(&mut self, warnings: LocationError) {
    self.warnings.push(warnings);
  }
  fn error(&mut self, error: LocationError) {
    self.errors.push(error);
  }
  fn debug_formatted_tokens(&mut self, file_name: String, tokens: AnilizedTokens) {
    self.tokens.insert(file_name, tokens);
  }
  fn debug_parsed_output(&mut self, _: String, _: String) {}
}
