pub mod utils;

#[cfg(test)]
mod tests;

use super::*;
use core::fmt::Display;
use files::{File, Path};
use std::collections::{HashMap, HashSet};
use std::fmt;
use tokenize::{
  Action, ActionType, Actions, Enum, Function, GlobalType, Import, Keywords, Struct, Type,
  TypeType, VarType, Variable,
};
use utils::{is_pascal_case, is_snake_case, GetLocation, GetName};

trait AddToAnylizeResults {
  fn add(self, add_to: &mut AnylizeResults);
}

#[derive(Clone)]
pub enum AnylizeErrAndWarns {
  // Warnings
  NameShouldBePascalCase, // SomeVarName
  NameShouldBeSnakeCase,  // some_var_name
  EmptyEnum,
  UnreachableCode,

  // Errors
  ContinueNotAllowed,
  BreakNotAllowed,
  NoName,
  NamingNotAllowed,
  NameAlreadyExists,
  AlreadyDefined,
  KeywordAsName,
  VariableRefDoesNotExist,
  FunctionDoesNotExist,
  VariableAlreadyDeclared,
  Inmutable,
}

impl AnylizeErrAndWarns {
  fn is_warning(&self) -> bool {
    match self {
      Self::NameShouldBePascalCase
      | Self::NameShouldBeSnakeCase
      | Self::EmptyEnum
      | Self::UnreachableCode => true,
      Self::NoName
      | Self::BreakNotAllowed
      | Self::ContinueNotAllowed
      | Self::NameAlreadyExists
      | Self::AlreadyDefined
      | Self::KeywordAsName
      | Self::NamingNotAllowed
      | Self::VariableRefDoesNotExist
      | Self::FunctionDoesNotExist
      | Self::VariableAlreadyDeclared
      | Self::Inmutable => false,
    }
  }
}

impl Display for AnylizeErrAndWarns {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      // Warnings
      Self::NameShouldBePascalCase => write!(
        f,
        "Name should be in pascal case (like this: SomeVariableName)"
      ),
      Self::NameShouldBeSnakeCase => write!(
        f,
        "Name should be in snake case (like this: some_variable_name)"
      ),
      Self::EmptyEnum => write!(f, "Empty enum"),
      Self::UnreachableCode => write!(f, "Unreachable code"),

      // Errors
      Self::BreakNotAllowed => write!(f, "Break not allowed here"),
      Self::ContinueNotAllowed => write!(f, "Continue not allowed here"),
      Self::AlreadyDefined => write!(f, "Already defined"),
      Self::NoName => write!(f, "No name provided"),
      Self::NameAlreadyExists => write!(f, "Name already exsits"),
      Self::NamingNotAllowed => write!(f, "A name is not allowed here"),
      Self::KeywordAsName => write!(f, "Using a language keyword is not allowed here"),
      Self::VariableRefDoesNotExist => write!(f, "The variable referenced doesn't exist"),
      Self::FunctionDoesNotExist => write!(f, "This function doesn't exist"),
      Self::VariableAlreadyDeclared => write!(f, "Variable already declared"),
      Self::Inmutable => write!(f, "Data in un mutatable"),
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
  pub imports: HashMap<String, Import>,
}

#[derive(Debug)]
struct SimpleAnilizedTokens<'a> {
  pub functions: &'a HashMap<String, Function>,
  pub vars: &'a HashMap<String, Variable>,
  pub structs: &'a HashMap<String, Struct>,
  pub enums: &'a HashMap<String, Enum>,
  pub types: &'a HashMap<String, GlobalType>,
  pub imports: &'a HashMap<String, Import>,
}

impl<'a> fmt::Debug for AnilizedTokens {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let simple = SimpleAnilizedTokens {
      functions: &self.functions,
      vars: &self.vars,
      structs: &self.structs,
      enums: &self.enums,
      types: &self.types,
      imports: &self.imports,
    };
    writeln!(f, "{:#?}", simple)
  }
}

pub fn anilize_tokens(
  compiler: &mut Compiler,
  tokenizer: Tokenizer,
) -> (AnilizedTokens, AnylizeResults) {
  let file = tokenizer.file;

  let mut anilized_res = AnylizeResults::new(file.clone());

  let mut used_keys: HashSet<String> = HashSet::new();

  let imports = array_into_hash_map(
    tokenizer.imports.clone(),
    &mut used_keys,
    SnakeOrPascal::Pascal,
    &mut anilized_res,
  );

  let functions = array_into_hash_map(
    tokenizer.functions.clone(),
    &mut used_keys,
    SnakeOrPascal::Snake,
    &mut anilized_res,
  );

  let vars = array_into_hash_map(
    tokenizer.vars.clone(),
    &mut used_keys,
    SnakeOrPascal::Snake,
    &mut anilized_res,
  );

  let structs = array_into_hash_map(
    tokenizer.structs.clone(),
    &mut used_keys,
    SnakeOrPascal::Pascal,
    &mut anilized_res,
  );

  let enums = array_into_hash_map(
    tokenizer.enums.clone(),
    &mut used_keys,
    SnakeOrPascal::Pascal,
    &mut anilized_res,
  );

  let types = array_into_hash_map(
    tokenizer.types.clone(),
    &mut used_keys,
    SnakeOrPascal::Pascal,
    &mut anilized_res,
  );

  let mut res = AnilizedTokens {
    file,
    functions,
    vars,
    structs,
    enums,
    types,
    imports,
  };
  anilized_res.check_anilized_tokens(compiler, &mut res);

  (res, anilized_res)
}

enum SnakeOrPascal {
  Snake,
  Pascal,
}

fn array_into_hash_map<T>(
  data: Vec<T>,
  used_keys: &mut HashSet<String>,
  name_should_be: SnakeOrPascal,
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

    if let SnakeOrPascal::Snake = name_should_be {
      if !is_snake_case(&name) {
        anilized_res.add(AnylizeErrAndWarns::NameShouldBeSnakeCase, &item.location());
      }
    } else if !is_pascal_case(&name) {
      anilized_res.add(AnylizeErrAndWarns::NameShouldBePascalCase, &item.location());
    }

    used_keys.insert(name.clone());
    res.insert(name, item);
  }

  res
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

  fn check_anilized_tokens(&mut self, compiler: &mut Compiler, data: &mut AnilizedTokens) {
    if data.imports.len() > 0 {
      let mut path = Path::from(&self.file.name);
      path.pop(); // Remove the filename from the path

      for (_, import) in data.imports.clone() {
        let mut cloned_path = path.clone();
        cloned_path.push(import.path.content);

        compiler.add_work(Work::ParseFile(cloned_path))
      }
    }

    // Check the global functions
    for (_, function) in data.functions.clone() {
      let mut check_state = CheckActionState::new(data);

      if function.args.len() > 0 {
        // check the function arguments
        let mut used_arg_names: Vec<String> = vec![];
        for (arg_name, arg_type) in function.args {
          if used_arg_names.contains(&arg_name) {
            // TODO: use the location of the name here
            self.add(AnylizeErrAndWarns::AlreadyDefined, &function.location);
            continue;
          }

          if !is_snake_case(&arg_name) {
            // TODO: use the location of the name here
            self.add(
              AnylizeErrAndWarns::NameShouldBeSnakeCase,
              &function.location,
            );
          }
          used_arg_names.push(arg_name.clone());
          check_state.vars.insert(
            arg_name,
            VariableDetials {
              global: false,
              mutatable: false,
            },
          );

          self.check_type(arg_type);
        }
      }

      if let Some(name) = &function.name {
        // Check if the function name is snake case
        if !is_snake_case(name) {
          self.add(
            AnylizeErrAndWarns::NameShouldBeSnakeCase,
            &function.location,
          );
        }
      }

      for (var_name, _) in &data.vars {
        check_state.vars.insert(
          var_name.clone(),
          VariableDetials {
            global: true,
            mutatable: false,
          },
        );
      }

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
          self.add(AnylizeErrAndWarns::AlreadyDefined, &field.location);
          continue;
        }
        used_field_names.push(field.name.clone());
        if !is_snake_case(&field.name) {
          self.add(AnylizeErrAndWarns::NameShouldBeSnakeCase, &field.location);
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
    for field in struct_.fields {
      if used_names.contains(&field.name) {
        // Check if the struct name issn't already used
        self.add(AnylizeErrAndWarns::NameAlreadyExists, &field.location);
        continue;
      }
      used_names.push(field.name.clone());
      if !is_snake_case(&field.name) {
        // Check if the struct field is snake case
        self.add(AnylizeErrAndWarns::NameShouldBeSnakeCase, &field.location);
      }

      self.check_type(field.type_);
    }

    if let Some(name) = &struct_.name {
      if is_inline {
        self.add(AnylizeErrAndWarns::NamingNotAllowed, &struct_.location);
        return;
      }

      // check if the struct name is in snake case
      if !is_pascal_case(name) {
        self.add(
          AnylizeErrAndWarns::NameShouldBePascalCase,
          &struct_.location,
        );
      }
    } else if !is_inline {
      self.add(AnylizeErrAndWarns::NoName, &struct_.location);
      return;
    }
  }

  fn check_actions(&mut self, actions: Actions, state: &mut CheckActionState) {
    let mut new_state = state.clone();
    for action in actions.actions {
      self.check_action(action, &mut new_state)
    }
  }

  fn check_action(&mut self, action: Action, state: &mut CheckActionState) {
    // TODO: Disallow some things when this is a inline action

    if state.unreachable_code {
      self.add(AnylizeErrAndWarns::UnreachableCode, &action.location);
    }

    match action.type_ {
      ActionType::Variable(var) => {
        if let Some(already_defined_var) = state.vars.get(&var.name) {
          if !already_defined_var.global {
            self.add(
              AnylizeErrAndWarns::VariableAlreadyDeclared,
              &action.location,
            );
            return;
          }
        }

        state.vars.insert(
          var.name,
          VariableDetials {
            global: false,
            mutatable: if let VarType::Let = var.var_type {
              true
            } else {
              false
            },
          },
        );

        self.check_action(*var.action, state);
      }
      ActionType::Return(data) => {
        // TODO: Check if this function actually expects response data
        if let Some(action) = data {
          self.check_action(*action, state);
        }

        state.unreachable_code = true;
      }
      ActionType::Assigment(data) => {
        if let Some(var) = state.vars.get(&data.name) {
          if !var.mutatable {
            self.add(AnylizeErrAndWarns::Inmutable, &action.location);
          }
        } else {
          self.add(
            AnylizeErrAndWarns::VariableRefDoesNotExist,
            &action.location,
          );
        }
        self.check_action(*data.action, state);
      }
      ActionType::FunctionCall(data) => {
        if !state.anilized_tokens.functions.contains_key(&data.name) {
          self.add(AnylizeErrAndWarns::FunctionDoesNotExist, &action.location);
        }

        for argument in &data.arguments {
          // TODO make sure these actions are checked inline and check if they match the expted function type
          self.check_action(argument.clone(), state);
        }
      }
      ActionType::VarRef(var_name) => {
        if !state.vars.contains_key(&var_name) {
          self.add(
            AnylizeErrAndWarns::VariableRefDoesNotExist,
            &action.location,
          );
          return;
        }
        // TODO: Check if the variable matches the expected type here if we expect some kind of type like function calls arguments
      }
      ActionType::StaticBoolean(_) => {
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

        state.unreachable_code = true;
      }
      ActionType::Continue => {
        if !state.inside_a_loop {
          self.add(AnylizeErrAndWarns::ContinueNotAllowed, &action.location)
        }

        state.unreachable_code = true;
      }
      ActionType::For(data) => {
        // TODO: Check if the variable matches the expected type
        self.check_action(*data.list, state);

        if let Some(var) = state.vars.get(&data.item_name) {
          if !var.global {
            self.add(
              AnylizeErrAndWarns::VariableAlreadyDeclared,
              &action.location,
            );
          }
        }

        state.inside_a_loop = true;
        self.check_actions(data.actions, state);
      }
      ActionType::While(data) => {
        // TODO: Check if the variable matches the expected type here (bool)
        self.check_action(*data.true_value, state);

        state.inside_a_loop = true;
        self.check_actions(data.actions, state);
      }
      ActionType::Loop(actions) => {
        state.inside_a_loop = true;
        self.check_actions(actions, state);
      }
      ActionType::If(data) => {
        // TODO: We can check a lot of things here like if we can never reach else ifs or else, and there are meany more

        // TODO: Check if the variable matches the expected type here
        self.check_action(*data.if_.check, state);
        self.check_actions(data.if_.body, state);

        for else_if in data.else_ifs {
          // TODO: Check if the variable matches the expected type here
          self.check_action(*else_if.check, state);
          self.check_actions(else_if.body, state);
        }

        if let Some(else_body) = data.else_body {
          self.check_actions(else_body, state);
        }
      }
    }
  }
}

#[derive(Clone)]
struct CheckActionState<'a> {
  inside_a_loop: bool,
  unreachable_code: bool,
  vars: HashMap<String, VariableDetials>,
  anilized_tokens: &'a AnilizedTokens,
}

#[derive(Clone)]
struct VariableDetials {
  global: bool,
  mutatable: bool,
}

impl<'a> CheckActionState<'a> {
  fn new(anilized_tokens: &'a AnilizedTokens) -> Self {
    Self {
      inside_a_loop: false,
      unreachable_code: false,
      vars: HashMap::new(),
      anilized_tokens,
    }
  }
}
