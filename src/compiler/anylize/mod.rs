mod utils;

#[cfg(test)]
mod tests;

use super::*;
use core::fmt::Display;
use std::collections::HashMap;
use std::fmt;
use tokenize::{Enum, Function, GlobalType, Keywords, Struct, Variable};
use utils::{is_camel_case, is_snake_case, GetName};

trait AddToAnylizeResults {
  fn add(self, add_to: &mut AnylizeResults);
}

#[derive(Clone)]
pub enum AnylizeWarning {
  NameShouldBeCamelCase,
  NameShouldBeSnakeCase,
}

impl Display for AnylizeWarning {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::NameShouldBeCamelCase => write!(f, "Name should be in camel case"),
      Self::NameShouldBeSnakeCase => write!(f, "Name should be in snake case"),
    }
  }
}

impl AddToAnylizeResults for AnylizeWarning {
  fn add(self, add_to: &mut AnylizeResults) {
    add_to.warnings.push(self);
  }
}

#[derive(Clone)]
pub enum AnylizeError {
  NoName,
  NameAlreadyExists,
  KeywordAsName,
}

impl Display for AnylizeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::NoName => write!(f, "No name provided"),
      Self::NameAlreadyExists => write!(f, "Name already exsits"),
      Self::KeywordAsName => write!(f, "Using a language keyword as name"),
    }
  }
}

impl AddToAnylizeResults for AnylizeError {
  fn add(self, add_to: &mut AnylizeResults) {
    add_to.errors.push(self);
  }
}

pub struct AnylizeResults {
  pub warnings: Vec<AnylizeWarning>,
  pub errors: Vec<AnylizeError>,
}

impl AnylizeResults {
  fn new() -> Self {
    Self {
      warnings: vec![],
      errors: vec![],
    }
  }
  fn add<T>(&mut self, item: T)
  where
    T: AddToAnylizeResults,
  {
    item.add(self);
  }
  fn merge(&mut self, merge_with: &mut Self) {
    self.errors.append(&mut merge_with.errors);
    self.warnings.append(&mut merge_with.warnings);
  }
}

#[derive(Clone)]
pub struct AnilizedTokens {
  file_name: Option<String>,
  pub functions: HashMap<String, Function>,
  pub vars: HashMap<String, Variable>,
  pub structs: HashMap<String, Struct>,
  pub enums: HashMap<String, Enum>,
  pub types: HashMap<String, GlobalType>,
}

#[derive(Debug)]
struct SimpleAnilizedTokens<'a> {
  pub file_name: &'a Option<String>,
  pub functions: &'a HashMap<String, Function>,
  pub vars: &'a HashMap<String, Variable>,
  pub structs: &'a HashMap<String, Struct>,
  pub enums: &'a HashMap<String, Enum>,
  pub types: &'a HashMap<String, GlobalType>,
}

impl fmt::Debug for AnilizedTokens {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let simple = SimpleAnilizedTokens {
      file_name: &self.file_name,
      functions: &self.functions,
      vars: &self.vars,
      structs: &self.structs,
      enums: &self.enums,
      types: &self.types,
    };
    writeln!(f, "{:#?}", simple)
  }
}

pub fn anilize_tokens(tokenizer: &Tokenizer) -> (AnilizedTokens, AnylizeResults) {
  let mut anilized_res = AnylizeResults::new();

  let file_name = tokenizer.get_file_name();

  let (functions, mut functions_res) =
    array_into_hash_map(tokenizer.functions.clone(), SnakeOrCamel::Snake);
  anilized_res.merge(&mut functions_res);

  let (vars, mut vars_res) = array_into_hash_map(tokenizer.vars.clone(), SnakeOrCamel::Snake);
  anilized_res.merge(&mut vars_res);

  let (structs, mut structs_res) =
    array_into_hash_map(tokenizer.structs.clone(), SnakeOrCamel::Camel);
  anilized_res.merge(&mut structs_res);

  let (enums, mut enums_res) = array_into_hash_map(tokenizer.enums.clone(), SnakeOrCamel::Camel);
  anilized_res.merge(&mut enums_res);

  let (types, mut types_res) = array_into_hash_map(tokenizer.types.clone(), SnakeOrCamel::Camel);
  anilized_res.merge(&mut types_res);

  let res = AnilizedTokens {
    file_name,
    functions,
    vars,
    structs,
    enums,
    types,
  };
  (res, anilized_res)
}

enum SnakeOrCamel {
  Snake,
  Camel,
}

fn array_into_hash_map<T>(
  data: Vec<T>,
  name_should_be: SnakeOrCamel,
) -> (HashMap<String, T>, AnylizeResults)
where
  T: GetName,
{
  let mut anilized_res = AnylizeResults::new();
  let mut res: HashMap<String, T> = HashMap::new();
  for item in data {
    let name = if let Some(name) = item.name() {
      name
    } else {
      anilized_res.add(AnylizeError::NoName);
      continue;
    };

    if res.contains_key(&name) {
      anilized_res.add(AnylizeError::NameAlreadyExists);
      continue;
    }

    if Keywords::is_keyword(&name) {
      anilized_res.add(AnylizeError::KeywordAsName);
      continue;
    }

    if let SnakeOrCamel::Snake = name_should_be {
      if !is_snake_case(&name) {
        anilized_res.add(AnylizeWarning::NameShouldBeSnakeCase);
        continue;
      }
    } else {
      if !is_camel_case(&name) {
        anilized_res.add(AnylizeWarning::NameShouldBeCamelCase);
        continue;
      }
    }

    res.insert(name, item);
  }
  (res, anilized_res)
}

impl GetName for Function {
  fn name(&self) -> Option<String> {
    self.name.clone()
  }
}

impl GetName for Variable {
  fn name(&self) -> Option<String> {
    Some(self.name.clone())
  }
}

impl GetName for Struct {
  fn name(&self) -> Option<String> {
    self.name.clone()
  }
}

impl GetName for Enum {
  fn name(&self) -> Option<String> {
    self.name.clone()
  }
}

impl GetName for GlobalType {
  fn name(&self) -> Option<String> {
    Some(self.name.clone())
  }
}
