use super::*;

pub struct JSGenerator {
  pub src: String
}

impl JSGenerator {
  // Generate javascript code using tokens from parser
  pub fn generate(parser: Parser) -> Result<Self, ParsingError> {
    let mut code = Self{
      src: String::new()
    };
    // define functions
    for func in parser.functions {
      let function = code.function(func);
      code.src += &function;
    }
    for glob in parser.global_vars {
      let global = code.global(glob);
      code.src += &global;
    }
    // Because JS does not call main, we must do that here
    code.src += "main();\n";
    Ok(code)
  }
  pub fn function(&mut self, func: Function) -> String {
    let mut src = String::new();
    src += "function ";
    src += &func.name.unwrap();
    src += "(";
    let mut index = 0;
    while index < func.args.len() {
      let arg = &func.args[index];
      src += &arg.0;
      index += 1;
    }
    src += ") {\n";
    let actions = func.body.list;
    for action in actions {
      src += &self.action(action);
    }
    src += "}\n";
    return src.to_string();
  }
  pub fn global(&mut self, var: Variable) -> String {
    let mut src = String::new();
    match var.var_type {
      variable::VarType::Const => {src += "const ";}
      _ => {/* Add error */}
    }
    src += &var.name.to_string();
    src += " = ";
    src += &self.action(*var.action);
    src += ";\n";
    return src.to_string();
  }
  pub fn action(&mut self, action: Action) -> String {
    // match an action and return code
    let mut src = String::new();
    match action {
      Action::Break => src += "break;\n",
      Action::Continue => src += "continue;\n",
      Action::For(res) => src += &self.action_for(res),
      Action::FunctionCall(res) => src += &self.action_func_call(res),
      Action::Loop(res) => src += &self.action_loop(res),
      Action::StaticNumber(res) => src += &self.action_num(res),
      Action::StaticString(res) => src += &self.action_str(res),
      Action::Variable(res) => src += &self.action_var(res),
      Action::VarRef(res) => src += &res,
      _ => {src += "undefined\n"}
    }
    return src;
  }
  pub fn action_for(&mut self, action: action::ActionFor) -> String {
    let mut src = String::new();
    src += "for (";
    src += &action.item_name;
    src += " in ";
    src += &self.action(*action.list);
    src += ") {\n";
    for act in action.actions.list {
      src += &self.action(act);
    }
    src += "}\n";
    return src;
  }
  pub fn action_func_call(&mut self, action: action::ActionFunctionCall) -> String {
    let mut src = String::new();
    src += &action.name;
    src += "(";
    for arg in action.arguments {
      src += &self.action(arg);
      src += ",";
    }
    src.pop();
    src += ");\n";
    return src;
  }
  pub fn action_loop(&mut self, action: actions::Actions) -> String {
    let mut src = String::new();
    src += "while (true) {\n";
    for act in action.list {
      src += &self.action(act);
    }
    src += "}\n";
    return src;
  }
  pub fn action_num(&mut self, action: numbers::Number) -> String {
    match action {
      Number::Float(res) => return res.to_string(),
      Number::Int(res) => return res.to_string(),
    }
  }
  pub fn action_str(&mut self, action: strings::String_) -> String {
    let mut src = String::new();
    src += "\"";
    src += &action.content;
    src += "\"";
    return src;
  }
  pub fn action_var(&mut self, action: variable::Variable) -> String {
    let mut src = String::new();
    match action.var_type {
      variable::VarType::Const => src += "const ",
      variable::VarType::Let => src += "let ",
    }
    src += &action.name;
    src += " = ";
    src += &self.action(*action.action);
    src += ";\n";
    return src;
  }
}
