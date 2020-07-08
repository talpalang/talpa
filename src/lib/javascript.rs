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
    // TODO: define globals
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
      match action {
        Action::FunctionCall(res) => {src += &self.function_call(res)},
        Action::Variable(res) => {src += &self.variable(res)},
        // TODO: Add remaining actions
        _ => {println!("Not all types are supported yet");}
      }
    }
    src += "}\n";
    return src.to_string();
  }
  pub fn function_call(&mut self, call: action::ActionFunctionCall) -> String {
    let mut src = String::new();
    src += &call.name;
    src += "(";
    // TODO: generate args
    src += ");\n";
    return src;
  }
  pub fn variable(&mut self, var: Variable) -> String {
    let mut src = String::new();
    match var.var_type {
      variable::VarType::Let => {src += "var "},
      variable::VarType::Const => {src += "const "}
    }
    src += &var.name;
    src += " = ";
    // TODO: generate value
    src += ";\n";
    return src;
  }
}
