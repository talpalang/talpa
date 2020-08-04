use super::*;

pub struct Go {}

impl Go {
  /// Generate golang code using tokens from parser
  pub fn generate(lb: &mut LangBuilder, t: AnilizedTokens) -> Result<(), LocationError> {
    // TODO: Replace when file importing is implemented
    lb.code("package main");

    let mut code = Self {};

    // define functions
    for (_, func) in t.functions {
      code.function(func, lb);
    }

    // define types
    for (_, type_) in t.types {
      code.custom_type(type_, lb);
    }

    // define structs
    for (_, structure) in t.structs {
      code.structure(structure, lb);
    }

    // define globals
    for (_, glob) in t.vars {
      code.global_var(glob, lb);
    }

    Ok(())
  }
  /// Parse a type
  pub fn parse_type(&mut self, type_: Type, lb: &mut impl BuildItems) {
    match type_.type_ {
      TypeType::Array(res) => self.type_array(res, lb),
      TypeType::Char => lb.code("char"),
      TypeType::Int => lb.code("int"),
      TypeType::I8 => lb.code("int8"),
      TypeType::I16 => lb.code("int16"),
      TypeType::I32 => lb.code("int32"),
      TypeType::I64 => lb.code("int64"),
      TypeType::UInt => lb.code("uint"),
      TypeType::U8 => lb.code("uint8"),
      TypeType::U16 => lb.code("uint16"),
      TypeType::U32 => lb.code("uint32"),
      TypeType::U64 => lb.code("uint64"),
      TypeType::String => lb.code("string"),
      TypeType::Struct(res) => self.structure(res, lb),
      TypeType::TypeRef(res) => lb.code(res),
      TypeType::Enum(_) => unimplemented!(),
    }
  }
  /// Parse a custom type definition
  pub fn custom_type(&mut self, type_: GlobalType, lb: &mut impl BuildItems) {
    let mut code = Inline::from_str(format!("type {} ", type_.name));
    self.parse_type(type_.type_, &mut code);
    lb.inline(code);
  }
  /// Parse a function
  pub fn function(&mut self, func: Function, lb: &mut impl BuildItems) {
    let mut prefix = Inline::from_str(format!("func {}(", func.name.unwrap()));
    let mut is_first = true;
    for (name, type_) in func.args {
      if is_first {
        is_first = false;
      } else {
        prefix.code(", ");
      }
      prefix.code(format!("{} ", name));
      self.parse_type(type_, &mut prefix);
    }
    prefix.code(")");

    if let Some(type_) = func.res {
      prefix.code(" ");
      self.parse_type(type_, &mut prefix);
    }

    let mut actions = Block::new();
    for action in func.body.actions {
      self.action(action, &mut actions, false);
    }

    lb.function(prefix, actions);
  }
  /// Parse a const variable
  pub fn global_var(&mut self, var: Variable, lb: &mut impl BuildItems) {
    let mut inline = Inline::new();

    inline.code(format!("const {} = ", var.name.to_string()));
    self.action(*var.action, &mut inline, true);

    lb.inline(inline);
  }
  /// Parse a structure
  pub fn structure(&mut self, structure: Struct, lb: &mut impl BuildItems) {
    let prefix_str = if let Some(name) = structure.name {
      format!("type {} struct ", name)
    } else {
      String::from("struct ")
    };

    let mut fields = Block::new();
    for field in structure.fields {
      let mut contents = Inline::from_str(format!("{} ", field.0));
      self.parse_type(field.1, &mut contents);
      fields.inline(contents);
    }
    lb.function(Inline::from_str(prefix_str), fields);
  }
  /// Parse array type
  pub fn type_array(&mut self, item: Box<Type>, lb: &mut impl BuildItems) {
    let mut array = Inline::from_str("[]");
    self.parse_type(*item, &mut array);
    lb.inline(array);
  }
  /// Parse an action
  pub fn action(&mut self, action: Action, lb: &mut impl BuildItems, inline: bool) {
    // match an action and return code
    match action.type_ {
      ActionType::Assigment(res) if inline => {
        lb.code(res.name + " = ");
        self.action(*res.action, lb, true);
      }
      ActionType::Assigment(res) => {
        let mut inline = Inline::from_str(res.name + " = ");
        self.action(*res.action, &mut inline, true);
      }
      ActionType::Break => lb.code("break"),
      ActionType::Continue => lb.code("continue"),
      ActionType::For(res) => self.action_for(res, lb),
      ActionType::FunctionCall(res) => self.action_func_call(res, lb),
      ActionType::Loop(res) => self.action_loop(res, lb),
      ActionType::Return(res) => self.action_return(res, lb),
      ActionType::StaticNumber(res) => self.action_num(res, lb),
      ActionType::StaticString(res) => self.action_str(res, lb),
      ActionType::Variable(res) => self.action_var(res, lb),
      ActionType::VarRef(res) => lb.code(res),
      ActionType::While(res) => self.action_while(res, lb),
      ActionType::If(if_) => self.action_if(if_, lb),
    };
  }
  fn if_block(
    &mut self,
    lb: &mut impl BuildItems,
    body: Actions,
    prefix: &'static str,
    add_to_prefix: impl FnOnce(&mut Self, &mut Inline),
  ) {
    let mut prefix = Inline::from_str(prefix);
    add_to_prefix(self, &mut prefix);
    let mut actions = Block::new();
    for action in body.actions {
      self.action(action, &mut actions, false);
    }
    lb.function(prefix, actions);
  }
  pub fn action_if(&mut self, if_: ActionIf, lb: &mut impl BuildItems) {
    // if
    let segment = *if_.if_.check;
    let body = if_.if_.body.clone();
    self.if_block(lb, body, "if ", |s, p| {
      s.action(segment, p, true);
    });

    // else if
    for else_if in if_.else_ifs {
      self.if_block(lb, else_if.body.clone(), "else if ", |s, p| {
        s.action(*else_if.check, p, true);
      });
    }

    // else
    match if_.else_body {
      Some(res) => self.if_block(lb, res, "else", |_, _| {}),
      None => {}
    }
  }
  pub fn action_for(&mut self, action: ActionFor, lb: &mut impl BuildItems) {
    let mut prefix =
      Inline::from_str(format!("for _, {name} := range ", name = &action.item_name,));
    self.action(*action.list, &mut prefix, true);

    let mut actions = Block::new();
    for action in action.actions.actions {
      self.action(action, &mut actions, false);
    }

    lb.function(prefix, actions);
  }
  pub fn action_func_call(&mut self, action: ActionFunctionCall, lb: &mut impl BuildItems) {
    let mut src = Inline::from_str(action.name + "(");

    for (i, arg) in action.arguments.iter().enumerate() {
      if i != 0 {
        src.code(",");
      }
      self.action(arg.clone(), &mut src, true);
    }

    src.code(")");

    lb.inline(src);
  }
  pub fn action_loop(&mut self, action: Actions, lb: &mut impl BuildItems) {
    let prefix = Inline::from_str("for true");

    let mut contents = Block::new();
    for act in action.actions {
      self.action(act, &mut contents, false);
    }

    lb.function(prefix, contents);
  }
  pub fn action_return(&mut self, action: Option<Box<Action>>, lb: &mut impl BuildItems) {
    let to_add = if let Some(return_action) = action {
      let mut src = Inline::from_str("return ");
      self.action(*return_action, &mut src, true);
      src
    } else {
      Inline::from_str("return")
    };

    lb.inline(to_add);
  }
  pub fn action_num(&mut self, action: Number, lb: &mut impl BuildItems) {
    lb.code(match action.type_ {
      NumberType::Float(res) => res.to_string(),
      NumberType::Int(res) => res.to_string(),
    });
  }
  pub fn action_str(&mut self, action: String_, lb: &mut impl BuildItems) {
    lb.code(format!("\"{}\"", action.content));
  }
  pub fn action_var(&mut self, action: Variable, lb: &mut impl BuildItems) {
    let prefix = format!("{} := ", action.name);
    let mut src = Inline::from_str(prefix);

    self.action(*action.action, &mut src, true);

    lb.inline(src);
  }
  pub fn action_while(&mut self, action: ActionWhile, lb: &mut impl BuildItems) {
    let mut prefix = Inline::from_str("for ");
    self.action(*action.true_value, &mut prefix, true);

    let mut contents = Block::new();
    for action in action.actions.actions {
      self.action(action, &mut contents, false);
    }

    lb.function(prefix, contents);
  }
}
