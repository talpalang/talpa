mod utils;

#[cfg(test)]
mod tests;

use super::*;
use core::fmt::Display;
use files::File;
use std::collections::{HashMap, HashSet};
use std::fmt;
use tokenize::{Enum, Function, GlobalType, Keywords, Struct, TypeType, Variable};
use utils::{is_camel_case, is_snake_case, GetLocation, GetName};

trait AddToAnylizeResults {
  fn add(self, add_to: &mut AnylizeResults);
}

#[derive(Clone)]
pub enum AnylizeErrAndWarns {
  // Warnings
  NameShouldBeCamelCase,
  NameShouldBeSnakeCase,
  EmptyEnum,

  // Errors
  NoName,
  NameAlreadyExists,
  AlreadyDefined,
  KeywordAsName,
}

impl AnylizeErrAndWarns {
  fn is_warning(&self) -> bool {
    match self {
      Self::NameShouldBeCamelCase | Self::NameShouldBeSnakeCase | Self::EmptyEnum => true,
      Self::NoName | Self::NameAlreadyExists | Self::AlreadyDefined | Self::KeywordAsName => false,
    }
  }
}

impl Display for AnylizeErrAndWarns {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::NameShouldBeCamelCase => write!(f, "Name should be in camel case"),
      Self::NameShouldBeSnakeCase => write!(f, "Name should be in snake case"),
      Self::EmptyEnum => write!(f, "Empty enum"),

      Self::AlreadyDefined => write!(f, "Already defined"),
      Self::NoName => write!(f, "No name provided"),
      Self::NameAlreadyExists => write!(f, "Name already exsits"),
      Self::KeywordAsName => write!(f, "Using a language keyword as name"),
    }
  }
}

pub struct AnylizeResults {
  file: File,
  pub warnings: Vec<LocationError>,
  pub errors: Vec<LocationError>,
}

impl AnylizeResults {
  fn new(file: File) -> Self {
    Self {
      file,
      warnings: vec![],
      errors: vec![],
    }
  }
  fn add(&mut self, item: AnylizeErrAndWarns, location: CodeLocation) {
    let error = self.file.must_error(item.clone(), location);
    if item.is_warning() {
      self.warnings.push(error);
    } else {
      self.errors.push(error);
    }
  }
}

#[derive(Clone)]
pub struct AnilizedTokens {
  file: File,
  pub functions: HashMap<String, Function>,
  pub vars: HashMap<String, Variable>,
  pub structs: HashMap<String, Struct>,
  pub enums: HashMap<String, Enum>,
  pub types: HashMap<String, GlobalType>,
}

#[derive(Debug)]
struct SimpleAnilizedTokens<'a> {
  pub file: &'a File,
  pub functions: &'a HashMap<String, Function>,
  pub vars: &'a HashMap<String, Variable>,
  pub structs: &'a HashMap<String, Struct>,
  pub enums: &'a HashMap<String, Enum>,
  pub types: &'a HashMap<String, GlobalType>,
}

impl<'a> fmt::Debug for AnilizedTokens {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let simple = SimpleAnilizedTokens {
      file: &self.file,
      functions: &self.functions,
      vars: &self.vars,
      structs: &self.structs,
      enums: &self.enums,
      types: &self.types,
    };
    writeln!(f, "{:#?}", simple)
  }
}

pub fn anilize_tokens(tokenizer: Tokenizer) -> (AnilizedTokens, AnylizeResults) {
  let file = tokenizer.file;

  let mut anilized_res = AnylizeResults::new(file.clone());

  let mut used_keys: HashSet<String> = HashSet::new();

  let functions = array_into_hash_map(
    tokenizer.functions.clone(),
    &mut used_keys,
    SnakeOrCamel::Snake,
    &mut anilized_res,
  );

  let vars = array_into_hash_map(
    tokenizer.vars.clone(),
    &mut used_keys,
    SnakeOrCamel::Snake,
    &mut anilized_res,
  );

  let structs = array_into_hash_map(
    tokenizer.structs.clone(),
    &mut used_keys,
    SnakeOrCamel::Camel,
    &mut anilized_res,
  );

  let enums = array_into_hash_map(
    tokenizer.enums.clone(),
    &mut used_keys,
    SnakeOrCamel::Camel,
    &mut anilized_res,
  );

  let types = array_into_hash_map(
    tokenizer.types.clone(),
    &mut used_keys,
    SnakeOrCamel::Camel,
    &mut anilized_res,
  );

  let mut res = AnilizedTokens {
    file,
    functions,
    vars,
    structs,
    enums,
    types,
  };
  res.anilize(&mut anilized_res);

  (res, anilized_res)
}

enum SnakeOrCamel {
  Snake,
  Camel,
}

fn array_into_hash_map<T>(
  data: Vec<T>,
  used_keys: &mut HashSet<String>,
  name_should_be: SnakeOrCamel,
  anilized_res: &mut AnylizeResults,
) -> HashMap<String, T>
where
  T: GetName + GetLocation,
{
  let mut res: HashMap<String, T> = HashMap::new();
  for item in data {
    let name = if let Some(name) = item.name() {
      name
    } else {
      anilized_res.add(AnylizeErrAndWarns::NoName, item.location());
      continue;
    };

    if used_keys.contains(&name) {
      anilized_res.add(AnylizeErrAndWarns::NameAlreadyExists, item.location());
      continue;
    }

    if Keywords::is_keyword(&name) {
      anilized_res.add(AnylizeErrAndWarns::KeywordAsName, item.location());
      continue;
    }

    if let SnakeOrCamel::Snake = name_should_be {
      if !is_snake_case(&name) {
        anilized_res.add(AnylizeErrAndWarns::NameShouldBeSnakeCase, item.location());
        continue;
      }
    } else {
      if !is_camel_case(&name) {
        anilized_res.add(AnylizeErrAndWarns::NameShouldBeCamelCase, item.location());
        continue;
      }
    }

    used_keys.insert(name.clone());
    res.insert(name, item);
  }

  res
}

impl GetName for Function {
  fn name(&self) -> Option<String> {
    self.name.clone()
  }
}

impl GetLocation for Function {
  fn location(&self) -> CodeLocation {
    self.location.clone()
  }
}

impl GetName for Variable {
  fn name(&self) -> Option<String> {
    Some(self.name.clone())
  }
}

impl GetLocation for Variable {
  fn location(&self) -> CodeLocation {
    self.location.clone()
  }
}

impl GetName for Struct {
  fn name(&self) -> Option<String> {
    self.name.clone()
  }
}

impl GetLocation for Struct {
  fn location(&self) -> CodeLocation {
    self.location.clone()
  }
}

impl GetName for Enum {
  fn name(&self) -> Option<String> {
    self.name.clone()
  }
}

impl GetLocation for Enum {
  fn location(&self) -> CodeLocation {
    self.location.clone()
  }
}

impl GetName for GlobalType {
  fn name(&self) -> Option<String> {
    Some(self.name.clone())
  }
}

impl GetLocation for GlobalType {
  fn location(&self) -> CodeLocation {
    self.location.clone()
  }
}

impl AnilizedTokens {
  fn anilize(&mut self, res: &mut AnylizeResults) {
    for (_, function) in self.functions.clone() {
      if function.args.len() > 0 {
        // check the function arguments
        let mut used_arg_names: Vec<String> = vec![];
        for (name, arg) in function.args {
          if used_arg_names.contains(&name) {
            res.add(AnylizeErrAndWarns::AlreadyDefined, arg.location.clone());
          } else {
            used_arg_names.push(name);
          }
          self.check_type(arg.type_, res);
        }
      }
    }

    for (_, enum_) in self.enums.clone() {
      if enum_.fields.len() == 0 {
        res.add(AnylizeErrAndWarns::EmptyEnum, enum_.location.clone());
        continue;
      }

      // Check the enum fields
      let mut used_field_names: Vec<String> = vec![];
      for field in enum_.fields {
        if used_field_names.contains(&field.name) {
          res.add(AnylizeErrAndWarns::AlreadyDefined, enum_.location.clone());
        } else {
          used_field_names.push(field.name);
        }
      }
    }
  }
  fn check_type(&mut self, _: TypeType, _: &mut AnylizeResults) {
    // TODO
  }
}
