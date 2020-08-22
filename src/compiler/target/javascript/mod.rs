use super::*;

pub struct JavaScript {}

impl JavaScript {
  // Generate javascript code using tokens from parser
  pub fn generate(lb: &mut LangBuilder, t: AnilizedTokens) -> Result<(), LocationError> {
    let mut code = Self {};

    // define functions
    for (_, func) in t.functions {
      code.function(func, lb);
    }
    // define global vars
    for (_, glob) in t.vars {
      code.global_var(glob, lb);
    }

    // Because JS does not call main, we must do that here
    lb.code("main();");
    Ok(())
  }
  pub fn function(&mut self, func: Function, lb: &mut impl BuildItems) {
    let mut prefix_str = format!("function {}(", func.name.unwrap());
    let mut args = vec![];
    for (name, _) in func.args {
      args.push(name);
    }
    prefix_str += &args.join(", ");
    prefix_str += ")";
    // prefix looks somwthing like this here
    // function foo(a, b, c)

    let mut actions = Block::new();
    for action in func.body.actions {
      self.action(action, &mut actions, false);
    }

    lb.function(Inline::from_str(prefix_str), actions);
  }
  pub fn global_var(&mut self, var: Variable, lb: &mut impl BuildItems) {
    let mut inline = Inline::new();

    inline.code(format!("const {} = ", var.name.to_string()));
    self.action(*var.action, &mut inline, true);
    inline.code(";");

    lb.inline(inline);
  }
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
      ActionType::Break => lb.code(if inline { "break" } else { "break;" }),
      ActionType::Continue => lb.code(if inline { "continue" } else { "continue;" }),
      ActionType::For(res) => self.action_for(res, lb),
      ActionType::FunctionCall(res) => self.action_func_call(res, lb, inline),
      ActionType::Loop(res) => self.action_loop(res, lb),
      ActionType::Return(res) => self.action_return(res, lb),
      ActionType::StaticNumber(res) => self.action_num(res, lb),
      ActionType::StaticString(res) => self.action_str(res, lb),
      ActionType::StaticBoolean(res) => self.action_bool(res, lb),
      ActionType::Variable(res) => self.action_var(res, lb),
      ActionType::VarRef(res) => lb.code(res + if inline { "" } else { ";" }),
      ActionType::While(res) => self.action_while(res, lb),
      ActionType::If(if_) => self.action_if(if_, lb), // TODO: make this
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
    let check = *if_.if_.check;
    let body = if_.if_.body.clone();
    self.if_block(lb, body, "if (", |s, p| {
      s.action(check, p, true);
      p.code(")");
    });

    // else if
    for else_if in if_.else_ifs {
      self.if_block(lb, else_if.body.clone(), "else if (", |s, p| {
        s.action(*else_if.check, p, true);
        p.code(")");
      });
    }

    // else
    match if_.else_body {
      Some(res) => self.if_block(lb, res, "else", |_, _| {}),
      None => {}
    }
  }
  pub fn action_for(&mut self, action: ActionFor, lb: &mut impl BuildItems) {
    let mut prefix = Inline::from_str(format!("for ({name} in ", name = &action.item_name,));
    self.action(*action.list, &mut prefix, true);
    prefix.code(")");

    let mut actions = Block::new();
    for action in action.actions.actions {
      self.action(action, &mut actions, false);
    }

    lb.function(prefix, actions);
  }
  pub fn action_func_call(
    &mut self,
    action: ActionFunctionCall,
    lb: &mut impl BuildItems,
    inline: bool,
  ) {
    let mut src = Inline::from_str(action.name + "(");

    for (i, arg) in action.arguments.iter().enumerate() {
      if i != 0 {
        src.code(",");
      }
      self.action(arg.clone(), &mut src, true);
    }

    src.code(if inline { ")" } else { ");" });

    lb.inline(src);
  }
  pub fn action_loop(&mut self, action: Actions, lb: &mut impl BuildItems) {
    let prefix = Inline::from_str("while (true)");

    let mut contents = Block::new();
    for act in action.actions {
      self.action(act, &mut contents, false);
    }

    lb.function(prefix, contents);
  }
  pub fn action_return(&mut self, action: Option<Box<Action>>, lb: &mut impl BuildItems) {
    let mut src = Inline::from_str("return ");

    self.action(*action.unwrap(), &mut src, true);
    src.code(";");

    lb.inline(src);
  }
  pub fn action_num(&mut self, number: Number, lb: &mut impl BuildItems) {
    lb.code(match number.type_ {
      NumberType::Float(res) => res.to_string(),
      NumberType::Int(res) => res.to_string(),
    });
  }
  pub fn action_str(&mut self, action: String_, lb: &mut impl BuildItems) {
    lb.code(format!("\"{}\"", action.content));
  }
  pub fn action_bool(&mut self, boolean: Boolean, lb: &mut impl BuildItems) {
    lb.code(boolean.to_string());
  }
  pub fn action_var(&mut self, action: Variable, lb: &mut impl BuildItems) {
    let prefix = format!(
      "{var_type} {var_name} = ",
      var_type = if let VarType::Const = action.var_type {
        "const"
      } else {
        "let"
      },
      var_name = action.name
    );
    let mut src = Inline::from_str(prefix);

    self.action(*action.action, &mut src, true);
    src.code(";");

    lb.inline(src);
  }
  pub fn action_while(&mut self, action: tokenize::ActionWhile, lb: &mut impl BuildItems) {
    let mut prefix = Inline::from_str("while (");
    self.action(*action.true_value, &mut prefix, true);
    prefix.code(")");

    let mut contents = Block::new();
    for action in action.actions.actions {
      self.action(action, &mut contents, false);
    }

    lb.function(prefix, contents);
  }
}
