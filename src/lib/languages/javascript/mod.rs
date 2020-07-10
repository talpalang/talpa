use super::*;

pub struct JavaScript {
  pub src: String
}

impl JavaScript {
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
    src += &format!("function {}( ", func.name.unwrap());
    for arg in func.args {
      src += &arg.0;
      src += ",";
    }
    src.pop();
    src += ") {\n";
    let actions = func.body.list;
    for action in actions {
      src += &self.action(action);
    }
    src += "}\n";
    return src;
  }
  pub fn global(&mut self, var: Variable) -> String {
    let mut src = String::new();
    match var.var_type {
      variable::VarType::Const => {src += "const ";}
      _ => {/* Add error */}
    }
    src += &format!(
      "{name} = {action};\n", 
      name=&var.name.to_string(), 
      action=&self.action(*var.action)
    );
    return src;
  }
  pub fn action(&mut self, action: Action) -> String {
    // match an action and return code
    let mut src = String::new();
    let action_code = match action {
      Action::Break => "break;\n".to_string(),
      Action::Continue => "continue;\n".to_string(),
      Action::For(res) => self.action_for(res),
      Action::FunctionCall(res) => self.action_func_call(res),
      Action::Loop(res) => self.action_loop(res),
      Action::Return(res) => self.action_return(res),
      Action::StaticNumber(res) => self.action_num(res),
      Action::StaticString(res) => self.action_str(res),
      Action::Variable(res) => self.action_var(res),
      Action::VarRef(res) => res,
      // add remaining actions
      Action::Assigment(_res) => "/* Assignment is unimplemented */".to_string(),
      Action::NOOP => "/* NOOP is unimplemented */".to_string(),
      Action::While(_res) => "/* While is unimplemented */".to_string(),
    };
    src += &action_code;
    return src;
  }
  pub fn action_for(&mut self, action: action::ActionFor) -> String {
    let mut src = String::new();
    src += &format!("for ({name} in {iter}) {{\n", name = &action.item_name, iter = &self.action(*action.list));
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
  pub fn action_return(&mut self, action: Option<Box<Action>>) -> String {
    let mut src = String::new();
    src += "return ";
    src += &self.action(*action.unwrap());
    src += ";\n";
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
