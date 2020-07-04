use super::*;

#[derive(Debug)]
pub enum Action {
  Variable(Variable),
  Return(Option<Box<Action>>),
  Assigment(ActionAssigment),
  FunctionCall(ActionFunctionCall),
  VarRef(String),
  StaticString(String_),
  StaticNumber(Number),
}

#[derive(Debug)]
pub struct ActionAssigment {
  pub name: String,
  pub action: Box<Action>,
}

impl Into<Action> for ActionAssigment {
  fn into(self) -> Action {
    Action::Assigment(self)
  }
}

#[derive(Debug)]
pub struct ActionFunctionCall {
  pub name: String,
  pub arguments: Vec<Action>,
}

impl Into<Action> for ActionFunctionCall {
  fn into(self) -> Action {
    Action::FunctionCall(self)
  }
}

pub struct ParseAction<'a> {
  p: &'a mut Parser,
  res: Option<Action>,
  action_to_expect: ActionToExpect,
}

pub enum ParseActionState {
  Return(ParseActionStateReturn),             // return foo
  Assigment(ParseActionStateAssigment),       // foo = bar
  FunctionCall(ParseActionStateFunctionCall), // foo(bar)
  VarRef(String),                             // foo
}

pub struct ParseActionStateFunctionCall {
  name: String,
  arguments: Vec<Action>,
}

impl Into<ParseActionState> for ParseActionStateFunctionCall {
  fn into(self) -> ParseActionState {
    ParseActionState::FunctionCall(self)
  }
}

pub struct ParseActionStateAssigment {
  name: String,
  action: Option<Action>,
}

impl Into<ParseActionState> for ParseActionStateAssigment {
  fn into(self) -> ParseActionState {
    ParseActionState::Assigment(self)
  }
}

pub struct ParseActionStateReturn {
  action: Option<Action>, // The value to return
}

impl Into<ParseActionState> for ParseActionStateReturn {
  fn into(self) -> ParseActionState {
    ParseActionState::Return(self)
  }
}

#[derive(PartialEq)]
pub enum ActionToExpect {
  /// A line in a function body
  ActionInBody,
  /// A assingment of some sort,
  /// like the contents of a variable or a function argument or the value of the return
  ///
  /// The str argument tells to return Ok instaid of unexected char if on end of parsing
  /// The unexpected char must match some letter out of the argument string
  Assignment(&'static str),
}

enum DetectedAction {
  /// 1. Just a variable name `foo`
  VarRefName,
  /// 2. `foo = bar`
  Assignment,
  /// 3. functions `foo()`
  Function,
  // 4. inline strings `"foo"`
  // 5. inline numbers `1`
  // 6. inline arrays `[foo, bar]`
  // 7. inline structs `foo{bar: baz}`
}

impl<'a> ParseAction<'a> {
  pub fn start(
    p: &'a mut Parser,
    go_back_one: bool,
    action_to_expect: ActionToExpect,
  ) -> Result<Action, ParsingError> {
    if go_back_one {
      p.index -= 1;
    }
    let mut s = Self {
      action_to_expect,
      p,
      res: None,
    };
    s.detect()?;
    if let Some(res) = s.res {
      Ok(res)
    } else {
      s.p.error(ParsingErrorType::UnexpectedResult)
    }
  }
  fn commit_state(&mut self, state: impl Into<ParseActionState>) -> Result<(), ParsingError> {
    self.res = Some(match state.into() {
      ParseActionState::Return(meta) => {
        let mut return_action: Option<Box<Action>> = None;
        if let Some(action) = meta.action {
          return_action = Some(Box::new(action));
        }
        Action::Return(return_action)
      }
      ParseActionState::Assigment(meta) => {
        if let None = meta.action {
          return self
            .p
            .error(ParsingErrorType::Custom("Missing variable assignment"));
        }

        ActionAssigment {
          name: meta.name,
          action: Box::new(meta.action.unwrap()),
        }
        .into()
      }
      ParseActionState::FunctionCall(meta) => ActionFunctionCall {
        name: meta.name,
        arguments: meta.arguments,
      }
      .into(),
      ParseActionState::VarRef(name) => Action::VarRef(name),
    });
    Ok(())
  }

  fn detect(&mut self) -> Result<(), ParsingError> {
    let matched_res = if self.action_to_expect == ActionToExpect::ActionInBody {
      self.p.try_match(&[
        (Keywords::Const, " \t\n"),
        (Keywords::Let, " \t\n"),
        (Keywords::Return, "} \t\n"),
      ])
    } else {
      // Matching keywords is only allowed when inside the body
      None
    };

    // Try to match a keyword and react to it
    if let Some(matched) = matched_res {
      match matched {
        Keywords::Const | Keywords::Let => {
          // Go to parsing the variable
          let var_type = if let Keywords::Const = matched {
            VarType::Const
          } else {
            VarType::Let
          };
          let new_var = parse_var(self.p, Some(var_type))?;
          self.res = Some(new_var.into());
        }
        Keywords::Return => {
          // Go to parsing the return
          let to_commit = self.parse_return()?;
          self.commit_state(to_commit)?;
        }
        Keywords::Fn => return self.p.error(ParsingErrorType::UnexpectedResult),
      }
      return Ok(());
    }

    // We are in a wired state right now where a lot of things are possible like
    // 1. variable assgiment `foo` or `foo = bar` (the second one is not allowed when ActionToExpect is ActionInBody)
    // 2. functions `foo()`
    // 3. inline strings `"foo"`
    // 4. inline numbers `1`
    // 5. inline arrays `[foo, bar]`
    // 6. inline structs `foo{bar: baz}`
    //
    // The code underhere will detect what the action is,
    // TODO: 2, 3, 4, 5, 6
    let mut name = NameBuilder::new();
    let mut detected_action = DetectedAction::VarRefName;
    let mut next_char = self.p.next_char();
    let mut name_completed = false;

    while let Some(c) = next_char {
      match c {
        '"' if name.len() == 0 => {
          // Parse a static string
          let parsed = parse_static_str(self.p)?;
          self.res = Some(parsed.into());
          return Ok(());
        }
        ' ' | '\t' | '\n' => {
          if name.len() > 0 {
            name_completed = true;
          }
          // Else ignore this
        }
        '(' => {
          // Detected start of a function call
          detected_action = DetectedAction::Function;
          break;
        }
        '=' => {
          // Detected variable assigment
          detected_action = DetectedAction::Assignment;
          break;
        }
        _ if (legal_name_char(c) || c == '.') && !name_completed => name.push(c),
        c => {
          if let ActionToExpect::Assignment(valid_unexpted_chars) = self.action_to_expect {
            if valid_unexpted_chars.contains(c) {
              self.p.index -= 1;
              break;
            }
          }
          return self.p.unexpected_char(c);
        }
      }
      next_char = self.p.next_char();
    }
    if let None = next_char {
      return self.p.unexpected_eof();
    }

    if let Some(number_parser) = name.is_number(self.p) {
      // The defined name is actually a number
      let number = number_parser.result(NumberTypes::Auto)?;
      self.res = Some(number.into());
      return Ok(());
    }

    let name_string = name.to_string(self.p)?;

    // Do things relative to the detected action
    match detected_action {
      DetectedAction::VarRefName => {
        self.commit_state(ParseActionState::VarRef(name_string))?;
      }
      DetectedAction::Assignment => {
        let res = self.parse_var_assignment(name_string, true)?;
        self.commit_state(res)?;
      }
      DetectedAction::Function => {
        let res = self.parse_function(name_string, false)?;
        self.commit_state(res)?;
      }
    };
    return Ok(());
  }
  fn parse_function(
    &mut self,
    name: String,
    check_for_function_open_sign: bool,
  ) -> Result<ParseActionStateFunctionCall, ParsingError> {
    let mut res = ParseActionStateFunctionCall {
      name,
      arguments: vec![],
    };

    if check_for_function_open_sign {
      match self.p.next_char() {
        Some('(') => {} // This is what we exect. return no error
        Some(c) => return self.p.unexpected_char(c),
        None => return self.p.unexpected_eof(),
      }
    }

    loop {
      match self.p.next_while(" \t\n") {
        Some(')') | None => {
          self.p.index -= 1;
          break;
        }
        _ => {}
      }

      let action = ParseAction::start(self.p, true, ActionToExpect::Assignment(",)"))?;
      res.arguments.push(action);
      match self.p.next_while(" \t\n") {
        Some(',') => continue,
        _ => {
          self.p.index -= 1;
          break;
        }
      }
    }

    match self.p.next_while(" \t\n") {
      Some(')') => {} // This is what we exect. return no error
      Some(c) => return self.p.unexpected_char(c),
      None => return self.p.unexpected_eof(),
    }

    Ok(res)
  }
  fn parse_var_assignment(
    &mut self,
    name: String,
    check_for_equal_sign: bool,
  ) -> Result<ParseActionStateAssigment, ParsingError> {
    let mut res = ParseActionStateAssigment { name, action: None };

    if check_for_equal_sign {
      match self.p.next_while(" \t\n") {
        Some('=') => {}
        Some(c) => return self.p.unexpected_char(c),
        None => return self.p.unexpected_eof(),
      }
    }

    match self.p.next_while(" \t\n") {
      Some(_) => {
        let action = ParseAction::start(self.p, true, ActionToExpect::Assignment(""))?;
        res.action = Some(action);
      }
      None => return self.p.unexpected_eof(),
    }

    Ok(res)
  }
  fn parse_return(&mut self) -> Result<ParseActionStateReturn, ParsingError> {
    let mut res = ParseActionStateReturn { action: None };

    match self.p.next_while(" \t\n") {
      Some('}') => {}
      Some(_) => {
        let action = ParseAction::start(self.p, true, ActionToExpect::Assignment("}"))?;
        res.action = Some(action);
      }
      None => return self.p.unexpected_eof(),
    }
    Ok(res)
  }
}
