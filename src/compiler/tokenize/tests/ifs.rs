use super::*;

#[test]
fn test_if() {
  parse_str(
    r#"
      fn test() {
        if true {}
      }
    "#,
  );
}

#[test]
fn test_if_names() {
  let tokens = parse_str(
    r#"
      fn test(value bool) {
        if value {}
      }
    "#
  );
  match &tokens.functions[0].body.list[0].type_ {
    action::ActionType::If(if_, _, _) => {
      let act = &*if_.0;
      match &act.type_ {
        action::ActionType::VarRef(name) if name == "value" => {},
        _ => {
          panic!("{:?}", tokens);
        }
      }
    },
    _ => panic!("{:?}", tokens)
  }
}

#[test]
fn test_wired_if() {
  parse_str(
    r#"
      fn test() {
        if
        true{    }
      }
    "#,
  );
}

#[test]
fn test_invalid_if() {
  parse_str_fail(
    r#"
      fn test() {
        if {}
      }
    "#,
  );
}

#[test]
fn test_if_else() {
  parse_str(
    r#"
      fn test() {
        if true {} else {}
      }
    "#,
  );
}

#[test]
fn test_wired_if_else() {
  parse_str(
    r#"
      fn test() {
        if
        true{    }
        else
        {

        }
      }
    "#,
  );
}

#[test]
fn test_invalid_if_else() {
  parse_str_fail(
    r#"
      fn test() {
        if true {} else
      }
    "#,
  );
}

#[test]
fn test_if_else_if() {
  parse_str(
    r#"
      fn test() {
        if true {} else if true {}
      }
    "#,
  );
}

#[test]
fn test_wired_if_else_if() {
  parse_str(
    r#"
      fn test() {
        if
        true{    }
        else
        if true{

        }
      }
    "#,
  );
}

#[test]
fn test_invalid_if_else_if() {
  parse_str_fail(
    r#"
      fn test() {
        if true {} else if {}
      }
    "#,
  );
}

#[test]
fn test_if_else_if_else() {
  parse_str(
    r#"
      fn test() {
        if true {} else if true {} else {}
      }
    "#,
  );
}

#[test]
fn test_wired_if_else_if_else() {
  parse_str(
    r#"
      fn test() {
        if
        true{    }
        else
        if true{

        }else{}
      }
    "#,
  );
}

#[test]
fn test_invalid_if_else_if_else() {
  parse_str_fail(
    r#"
      fn test() {
        if true {} else if true {} else
      }
    "#,
  );
}
