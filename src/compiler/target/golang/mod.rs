use super::*;

pub struct Go {}

impl Go {
  /// Generate golang code using tokens from parser
  pub fn generate(lb: &mut LangBuilder, t: AnilizedTokens) -> Result<(), LocationError> {

    // TODO: Replace when file importing is allowed
    lb.code("package main");

    let mut code = Self {};

    // define functions
    for (_, func) in t.functions {
      code.function(func, lb);
    }
    for (_, glob) in t.vars {
      code.global_var(glob, lb);
    }

    Ok(())
  }
  /// Get the type as a string
  pub fn get_type(&mut self, type_: Option<tokenize::types::Type>) -> String {
    match type_ {
      Some(res) => {
        match res.type_ {
          tokenize::types::TypeType::Char => "string".to_string(),
          tokenize::types::TypeType::String => "string".to_string(),
          tokenize::types::TypeType::Int => "int".to_string(),
          tokenize::types::TypeType::TypeRef(res) => res,
          _ => unimplemented!()
        }
      },
      None => "".to_string()
    }
  }
  /// Parse a function
  pub fn function(&mut self, func: Function, lb: &mut impl BuildItems) {
    let mut prefix_str = format!("func {}(", func.name.unwrap());
    let mut args = vec![];
    for (name, _) in func.args {
      args.push(name);
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
  // Parse an action
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
    };
  }
  pub fn action_for(&mut self, action: ActionFor, lb: &mut impl BuildItems) {
    //   for _, v := range items {
    //     fmt.Println(v)
    // }
    let mut prefix = Inline::from_str(format!("for _, {name} := range ", name = &action.item_name,));
    self.action(*action.list, &mut prefix, true);

    let mut actions = Block::new();
    for action in action.actions.list {
      self.action(action, &mut actions, false);
    }

    lb.function(prefix, actions);
  }
  pub fn action_func_call(
    &mut self,
    action: ActionFunctionCall,
    lb: &mut impl BuildItems,
  ) {
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
    let mut src = Inline::from_str("return ");

    self.action(*action.unwrap(), &mut src, true);

    lb.inline(src);
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
    let prefix = format!(
      "{var_type} {var_name} = ",
      var_type = if let VarType::Const = action.var_type {
        "const"
      } else {
        "var"
      },
      var_name = action.name
    );
    let mut src = Inline::from_str(prefix);

    self.action(*action.action, &mut src, true);

    lb.inline(src);
  }
  pub fn action_while(&mut self, action: tokenize::ActionWhile, lb: &mut impl BuildItems) {
    let mut prefix = Inline::from_str("for ");
    self.action(*action.true_value, &mut prefix, true);

    let mut contents = Block::new();
    for action in action.actions.list {
      self.action(action, &mut contents, false);
    }

    lb.function(prefix, contents);
  }
}
