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
      code.code_type(type_, lb);
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
  /// Get the type as a string
  pub fn get_type(&mut self, type_: Option<Type>) -> String {
    match type_ {
      Some(res) => match res.type_ {
        TypeType::Char => "rune".to_string(),
        TypeType::String => "string".to_string(),
        TypeType::Struct(res) => self.type_struct(res),
        TypeType::Int => "int".to_string(),
        TypeType::I8 => "int8".to_string(),
        TypeType::I16 => "int16".to_string(),
        TypeType::I32 => "int32".to_string(),
        TypeType::I64 => "int64".to_string(),
        TypeType::UInt => "uint".to_string(),
        TypeType::U8 => "uint8".to_string(),
        TypeType::U16 => "uint16".to_string(),
        TypeType::U32 => "uint32".to_string(),
        TypeType::U64 => "uint64".to_string(),
        TypeType::TypeRef(res) => res,
        TypeType::Array(res) => self.type_array(res),
        TypeType::Enum(_) => unimplemented!(),
      },
      None => "".to_string(),
    }
  }
  /// Parse a custom type definition
  pub fn code_type(&mut self, type_: GlobalType, lb: &mut impl BuildItems) {
    lb.code(format!(
      "type {} {}",
      type_.name,
      self.get_type(Some(type_.type_))
    ));
  }
  /// Parse a function
  pub fn function(&mut self, func: Function, lb: &mut impl BuildItems) {
    let mut prefix_str = format!("func {}(", func.name.unwrap());
    let mut args = vec![];
    for (name, type_) in func.args {
      args.push(format!("{} {}", name, self.get_type(Some(type_))));
    }
    prefix_str += &args.join(", ");
    prefix_str += ")";

    prefix_str += " ";
    prefix_str += &self.get_type(func.res);

    let mut actions = Block::new();
    for action in func.body.list {
      self.action(action, &mut actions, false);
    }

    lb.function(Inline::from_str(prefix_str), actions);
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
      fields.code(format!("{} {}", field.0, self.get_type(Some(field.1))));
    }
    lb.function(Inline::from_str(prefix_str), fields);
  }
  /// Global and inline structs
  pub fn type_struct(&mut self, structure: Struct) -> String {
    let mut code = "struct {\n".to_string();
    for field in structure.fields {
      code += &format!("\t{} {}\n", field.0, self.get_type(Some(field.1)));
    }
    code += "}\n";
    code
  }
  /// Parse array type
  pub fn type_array(&mut self, item: Box<Type>) -> String {
    format!("[]{}", self.get_type(Some(*item)))
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
      ActionType::If(if_, else_if, else_) => self.action_if(if_, else_if, else_, lb),
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
    for action in body.list {
      self.action(action, &mut actions, false);
    }
    lb.function(prefix, actions);
  }
  pub fn action_if(
    &mut self,
    if_: (std::boxed::Box<Action>, Actions),
    else_ifs: std::vec::Vec<(Action, Actions)>,
    else_: std::option::Option<Actions>,
    lb: &mut impl BuildItems,
  ) {
    // if
    let segment = *if_.0;
    let body = if_.1;
    self.if_block(lb, body, "if ", |s, p| {
      s.action(segment, p, true);
    });

    // else if
    for else_if in else_ifs {
      self.if_block(lb, else_if.1.clone(), "else if ", |s, p| {
        s.action(else_if.0.clone(), p, true);
      });
    }

    // else
    match else_ {
      Some(res) => self.if_block(lb, res, "else", |_, _| {}),
      None => {}
    }
  }
  pub fn action_for(&mut self, action: ActionFor, lb: &mut impl BuildItems) {
    //   for _, v := range items {
    //     fmt.Println(v)
    // }
    let mut prefix =
      Inline::from_str(format!("for _, {name} := range ", name = &action.item_name,));
    self.action(*action.list, &mut prefix, true);

    let mut actions = Block::new();
    for action in action.actions.list {
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
    for act in action.list {
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
    for action in action.actions.list {
      self.action(action, &mut contents, false);
    }

    lb.function(prefix, contents);
  }
}
