mod utils;

#[cfg(test)]
mod tests;

use super::*;
use core::fmt::Display;
use files::File;
use std::collections::{HashMap, HashSet};
use std::fmt;
use tokenize::{
  Action, ActionType, Actions, Enum, Function, GlobalType, Keywords, Struct, Type, TypeType,
  Variable,
};
use utils::{is_camel_case, is_snake_case, is_var_name, GetLocation, GetName};

trait AddToAnylizeResults {
  fn add(self, add_to: &mut AnylizeResults);
}

#[derive(Clone)]
pub enum AnylizeErrAndWarns {
  // Warnings
  NameShouldBeCamelCase, // SomeVarName
  NameShouldBeSnakeCase, // some_var_name
  EmptyEnum,

  // Errors
  ContinueNotAllowed,
  BreakNotAllowed,
  NoName,
  NamingNotAllowed,
  NameAlreadyExists,
  AlreadyDefined,
  KeywordAsName,
}

impl AnylizeErrAndWarns {
  fn is_warning(&self) -> bool {
    match self {
      Self::NameShouldBeCamelCase | Self::NameShouldBeSnakeCase | Self::EmptyEnum => true,
      Self::NoName
      | Self::BreakNotAllowed
      | Self::ContinueNotAllowed
      | Self::NameAlreadyExists
      | Self::AlreadyDefined
      | Self::KeywordAsName
      | Self::NamingNotAllowed => false,
    }
  }
}

impl Display for AnylizeErrAndWarns {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::NameShouldBeCamelCase => write!(f, "Name should be in camel case"),
      Self::NameShouldBeSnakeCase => write!(f, "Name should be in snake case"),
      Self::EmptyEnum => write!(f, "Empty enum"),

      Self::BreakNotAllowed => write!(f, "Break not allowed here"),
      Self::ContinueNotAllowed => write!(f, "Continue not allowed here"),
      Self::AlreadyDefined => write!(f, "Already defined"),
      Self::NoName => write!(f, "No name provided"),
      Self::NameAlreadyExists => write!(f, "Name already exsits"),
      Self::NamingNotAllowed => write!(f, "A name is not allowed here"),
      Self::KeywordAsName => write!(f, "Using a lided"),
    }
  }
}

pub struct AnylizeResults {
  file: File,
  pub warnings: Vec<LocationError>,
  pub errors: Vec<LocationError>,
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
  pub functions: &'a HashMap<String, Function>,
  pub vars: &'a HashMap<String, Variable>,
  pub structs: &'a HashMap<String, Struct>,
  pub enums: &'a HashMap<String, Enum>,
  pub types: &'a HashMap<String, GlobalType>,
}

impl<'a> fmt::Debug for AnilizedTokens {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let simple = SimpleAnilizedTokens {
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
  anilized_res.check_anilized_tokens(&mut res);

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
      anilized_res.add(AnylizeErrAndWarns::NoName, &item.location());
      continue;
    };

    if used_keys.contains(&name) {
      anilized_res.add(AnylizeErrAndWarns::NameAlreadyExists, &item.location());
      continue;
    }

    if Keywords::is_keyword(&name) {
      anilized_res.add(AnylizeErrAndWarns::KeywordAsName, &item.location());
      continue;
    }

    if let SnakeOrCamel::Snake = name_should_be {
      if !is_snake_case(&name) {
        anilized_res.add(AnylizeErrAndWarns::NameShouldBeSnakeCase, &item.location());
        continue;
      }
    } else {
      if !is_camel_case(&name) {
        anilized_res.add(AnylizeErrAndWarns::NameShouldBeCamelCase, &item.location());
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

impl AnylizeResults {
  fn new(file: File) -> Self {
    Self {
      file,
      warnings: vec![],
      errors: vec![],
    }
  }
  fn add(&mut self, item: AnylizeErrAndWarns, location: &CodeLocation) {
    let error = self.file.must_error(item.clone(), location.clone());
    if item.is_warning() {
      self.warnings.push(error);
    } else {
      self.errors.push(error);
    }
  }

  fn check_anilized_tokens(&mut self, data: &mut AnilizedTokens) {
    // Check the global functions
    for (_, function) in data.functions.clone() {
      if function.args.len() > 0 {
        // check the function arguments
        let mut used_arg_names: Vec<String> = vec![];
        for (arg_name, arg_type) in function.args {
          if used_arg_names.contains(&arg_name) {
            // TODO: use the location of the name here
            self.add(AnylizeErrAndWarns::AlreadyDefined, &function.location);
            continue;
          }

          if !is_var_name(&arg_name) {
            // TODO: use the location of the name here
            self.add(
              AnylizeErrAndWarns::NameShouldBeSnakeCase,
              &function.location,
            );
          }
          used_arg_names.push(arg_name);
          self.check_type(arg_type);
        }
      }

      if let Some(name) = &function.name {
        // Check if the function name is snake case
        if !is_var_name(name) {
          self.add(
            AnylizeErrAndWarns::NameShouldBeSnakeCase,
            &function.location,
          );
        }
      }

      // TODO: Add the function args to the check_state
      let mut check_state = CheckActionState::new();

      self.check_actions(function.body, &mut check_state)
    }

    // Check the global enums
    for (_, enum_) in data.enums.clone() {
      if enum_.fields.len() == 0 {
        // TODO: Use the location of the fields here instaid of the enum
        self.add(AnylizeErrAndWarns::EmptyEnum, &enum_.location);
        continue;
      }

      // Check the enum fields
      let mut used_field_names: Vec<String> = vec![];
      for field in enum_.fields {
        if used_field_names.contains(&field.name) {
          // TODO: Use the location of the name here instaid of the enum
          self.add(AnylizeErrAndWarns::AlreadyDefined, &enum_.location);
          continue;
        }
        used_field_names.push(field.name.clone());
        if !is_var_name(&field.name) {
          // TODO: Use the location of the name here instaid of the enum
          self.add(AnylizeErrAndWarns::NameShouldBeSnakeCase, &enum_.location);
        }
      }
    }

    // Check the global structs
    for (_, struct_) in data.structs.clone() {
      self.check_struct(struct_, false);
    }
  }

  fn check_type(&mut self, type_: Type) {
    match type_.type_ {
      TypeType::Struct(struct_) => self.check_struct(struct_, true),
      TypeType::Array(array_type) => self.check_type(*array_type),
      _ => {}
    }
  }

  fn check_struct(&mut self, struct_: Struct, is_inline: bool) {
    let mut used_names: Vec<String> = vec![];
    for (field_name, field_type) in struct_.fields {
      if used_names.contains(&field_name) {
        // TODO: Use the location of the name here
        self.add(AnylizeErrAndWarns::NameAlreadyExists, &struct_.location);
        continue;
      }
      used_names.push(field_name.clone());
      if !is_var_name(&field_name) {
        // Check if the struct field is snake case
        // TODO: Use the location of the name here
        self.add(AnylizeErrAndWarns::NameShouldBeSnakeCase, &struct_.location);
      }

      self.check_type(field_type);
    }

    if let Some(name) = &struct_.name {
      if is_inline {
        self.add(AnylizeErrAndWarns::NamingNotAllowed, &struct_.location);
        return;
      }

      // check if the struct name is in snake case
      if !is_camel_case(name) {
        self.add(AnylizeErrAndWarns::NameShouldBeCamelCase, &struct_.location);
      }
    } else if !is_inline {
      self.add(AnylizeErrAndWarns::NoName, &struct_.location);
      return;
    }
  }

  fn check_actions(&mut self, actions: Actions, state: &mut CheckActionState) {
    for action in actions.actions {
      self.check_action(action, &mut state.clone())
    }
  }

  fn check_action(&mut self, action: Action, state: &mut CheckActionState) {
    match action.type_ {
      ActionType::Variable(_) => {
        // TODO: check this
      }
      ActionType::Return(_) => {
        // TODO: check this
      }
      ActionType::Assigment(_) => {
        // TODO: check this
      }
      ActionType::FunctionCall(_) => {
        // TODO: check this
      }
      ActionType::VarRef(_) => {
        // TODO: check this
      }
      ActionType::StaticString(_) => {
        // TODO: check this
      }
      ActionType::StaticNumber(_) => {
        // TODO: check this
      }
      ActionType::Break => {
        if !state.inside_a_loop {
          self.add(AnylizeErrAndWarns::BreakNotAllowed, &action.location)
        }
      }
      ActionType::Continue => {
        if !state.inside_a_loop {
          self.add(AnylizeErrAndWarns::ContinueNotAllowed, &action.location)
        }
      }
      ActionType::For(data) => {
        // TODO: Check list (Box<Action>) and item_name (String)
        state.inside_a_loop = true;
        self.check_actions(data.actions, state);
      }
      ActionType::While(data) => {
        // TODO: Check true_value (Box<Action>)
        state.inside_a_loop = true;
        self.check_actions(data.actions, state);
      }
      ActionType::Loop(actions) => {
        state.inside_a_loop = true;
        self.check_actions(actions, state);
      }
      ActionType::If(_) => {
        // TODO: check this
      }
    }
  }
}

#[derive(Clone)]
struct CheckActionState {
  inside_a_loop: bool,
}

impl CheckActionState {
  fn new() -> Self {
    Self {
      inside_a_loop: false,
    }
  }
}
