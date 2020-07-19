use super::*;
use actions::ParseActions;
use errors::{LocationError, TokenizeError};
use numbers::NumberTypes;
use statics::{valid_name_char, NameBuilder};
use strings::parse_static_str;
use variable::parse_var;

#[derive(Debug, Clone)]
pub enum Action {
  Variable(Variable),
  Return(Option<Box<Action>>),
  Assigment(ActionAssigment),
  FunctionCall(ActionFunctionCall),
  VarRef(String),
  StaticString(String_),
  StaticNumber(Number),
  Break,
  Continue,
  For(ActionFor),
  While(ActionWhile),
  Loop(Actions),
}

#[derive(Debug, Clone)]
pub struct ActionAssigment {
  pub name: String,
  pub action: Box<Action>,
}

impl Into<Action> for ActionAssigment {
  fn into(self) -> Action {
    Action::Assigment(self)
  }
}

#[derive(Debug, Clone)]
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
  t: &'a mut Tokenizer,
  res: Option<Action>,
  action_to_expect: ActionToExpect,
}

pub enum ParseActionState {
  Return(ParseActionStateReturn),             // return foo
  Assigment(ParseActionStateAssigment),       // foo = bar
  FunctionCall(ParseActionStateFunctionCall), // foo(bar)
  VarRef(String),                             // foo
  Break,
  Continue,
  For(ActionFor),
  While(ActionWhile),
  Loop(Actions),
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

enum LoopType {
  For,
  While,
  Loop,
}

impl Into<LoopType> for Keywords {
  fn into(self) -> LoopType {
    match self {
      Self::For => LoopType::For,
      Self::While => LoopType::While,
      _ => LoopType::Loop,
    }
  }
}

#[derive(Debug, Clone)]
pub struct ActionWhile {
  pub actions: Actions,
  pub true_value: Box<Action>,
}

impl Into<Action> for ActionWhile {
  fn into(self) -> Action {
    Action::While(self)
  }
}

#[derive(Debug, Clone)]
pub struct ActionFor {
  pub actions: Actions,
  pub list: Box<Action>,
  pub item_name: String,
}

impl Into<Action> for ActionFor {
  fn into(self) -> Action {
    Action::For(self)
  }
}

impl<'a> ParseAction<'a> {
  pub fn start(
    t: &'a mut Tokenizer,
    go_back_one: bool,
    action_to_expect: ActionToExpect,
  ) -> Result<Action, LocationError> {
    if go_back_one {
      t.index -= 1;
    }
    let mut s = Self {
      action_to_expect,
      t,
      res: None,
    };
    s.detect()?;
    if let Some(res) = s.res {
      Ok(res)
    } else {
      s.t.error(TokenizeError::UnexpectedResult)
    }
  }
  fn commit_state(&mut self, state: impl Into<ParseActionState>) -> Result<(), LocationError> {
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
            .t
            .error(TokenizeError::Custom("Missing variable assignment"));
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
      ParseActionState::Break => Action::Break,
      ParseActionState::Continue => Action::Continue,
      ParseActionState::While(meta) => meta.into(),
      ParseActionState::For(meta) => meta.into(),
      ParseActionState::Loop(actions) => Action::Loop(actions),
    });
    Ok(())
  }

  fn detect(&mut self) -> Result<(), LocationError> {
    let matched_res = if self.action_to_expect == ActionToExpect::ActionInBody {
      self.t.try_match(vec![
        &Keywords::Const,
        &Keywords::Let,
        &Keywords::Return,
        &Keywords::Loop,
        &Keywords::While,
        &Keywords::For,
        &Keywords::Break,
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
          let new_var = parse_var(self.t, Some(var_type))?;
          self.res = Some(new_var.into());
        }
        Keywords::Return => {
          // Go to parsing the return
          let to_commit = self.parse_return()?;
          self.commit_state(to_commit)?;
        }
        Keywords::Loop | Keywords::While | Keywords::For => {
          // Parse loop
          let to_commit = self.parse_looper(matched.clone().into())?;
          self.commit_state(to_commit)?;
        }
        Keywords::Break => self.commit_state(ParseActionState::Break)?,
        Keywords::Continue => self.commit_state(ParseActionState::Continue)?,
        Keywords::Fn | Keywords::Struct | Keywords::Enum | Keywords::Type => {
          return self.t.error(TokenizeError::UnexpectedResult)
        }
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
    let mut name_completed = false;

    while let Some(c) = self.t.next_char() {
      match c {
        '"' if name.len() == 0 => {
          // Parse a static string
          let parsed = parse_static_str(self.t)?;
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
        _ if (valid_name_char(c) || c == '.') && !name_completed => name.push(c),
        c => {
          if name_completed {
            self.t.index -= 1;
            break;
          }

          if let ActionToExpect::Assignment(valid_unexpted_chars) = self.action_to_expect {
            if valid_unexpted_chars.contains(c) {
              self.t.index -= 1;
              break;
            }
          }
          return self.t.unexpected_char(c);
        }
      }
    }

    if let Some(number_parser) = name.is_number(self.t) {
      // The defined name is actually a number
      let number = number_parser.result(NumberTypes::Auto)?;
      self.res = Some(number.into());
      return Ok(());
    }

    let name_string = name.to_string(self.t)?;

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
  ) -> Result<ParseActionStateFunctionCall, LocationError> {
    let mut res = ParseActionStateFunctionCall {
      name,
      arguments: vec![],
    };

    if check_for_function_open_sign {
      match self.t.must_next_char()? {
        '(' => {} // This is what we expect. return no error
        c => return self.t.unexpected_char(c),
      }
    }

    loop {
      match self.t.next_while(" \t\n") {
        Some(')') | None => {
          self.t.index -= 1;
          break;
        }
        _ => {}
      }

      let action = ParseAction::start(self.t, true, ActionToExpect::Assignment(",)"))?;
      res.arguments.push(action);
      match self.t.next_while(" \t\n") {
        Some(',') => continue,
        _ => {
          self.t.index -= 1;
          break;
        }
      }
    }

    match self.t.must_next_while(" \t\n")? {
      ')' => {} // This is what we exect. return no error
      c => return self.t.unexpected_char(c),
    }

    Ok(res)
  }
  fn parse_var_assignment(
    &mut self,
    name: String,
    check_for_equal_sign: bool,
  ) -> Result<ParseActionStateAssigment, LocationError> {
    let mut res = ParseActionStateAssigment { name, action: None };

    if check_for_equal_sign {
      match self.t.must_next_while(" \t\n")? {
        '=' => {}
        c => return self.t.unexpected_char(c),
      }
    }

    self.t.must_next_while(" \t\n")?;
    let action = ParseAction::start(self.t, true, ActionToExpect::Assignment(""))?;
    res.action = Some(action);

    Ok(res)
  }
  fn parse_looper(&mut self, loop_type: LoopType) -> Result<ParseActionState, LocationError> {
    self.t.must_next_while(" \t\n")?;

    let mut for_item_name: Option<String> = None;

    // Parse the bit between the "for"/"while" and "{"
    let loop_based_on = match loop_type {
      LoopType::While => Some(ParseAction::start(
        self.t,
        true,
        ActionToExpect::Assignment("{"),
      )?),
      LoopType::For => {
        let mut name = NameBuilder::new();
        loop {
          match self.t.must_next_char()? {
            ' ' | '\t' | '\n' => break,
            c if valid_name_char(c) => name.push(c),
            c => return self.t.unexpected_char(c),
          }
        }

        for_item_name = Some(name.to_string(self.t)?);
        self.t.expect("in")?;

        self.t.must_next_while(" \t\n")?;

        Some(ParseAction::start(
          self.t,
          true,
          ActionToExpect::Assignment("{"),
        )?)
      }
      LoopType::Loop => {
        self.t.index -= 1;
        None
      }
    };

    match self.t.must_next_while(" \t\n")? {
      '{' => {}
      c => return self.t.unexpected_char(c),
    };

    let actions = ParseActions::start(self.t)?;

    Ok(match loop_type {
      LoopType::For => ParseActionState::For(ActionFor {
        actions,
        list: Box::new(loop_based_on.unwrap()),
        item_name: for_item_name.unwrap_or(String::new()),
      }),
      LoopType::While => ParseActionState::While(ActionWhile {
        actions,
        true_value: Box::new(loop_based_on.unwrap()),
      }),
      LoopType::Loop => ParseActionState::Loop(actions),
    })
  }
  fn parse_return(&mut self) -> Result<ParseActionStateReturn, LocationError> {
    let mut res = ParseActionStateReturn { action: None };

    match self.t.must_next_while(" \t\n")? {
      '}' => {}
      _ => {
        let action = ParseAction::start(self.t, true, ActionToExpect::Assignment("}"))?;
        res.action = Some(action);
      }
    }
    Ok(res)
  }
}
