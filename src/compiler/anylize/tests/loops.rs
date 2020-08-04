use super::*;

#[test]
fn test_simple_for() {
  parse_str(
    r#"
      fn test(items []string) {
        for item in items {}
      }
    "#,
  );
}

#[test]
fn test_multiple_simple_fors() {
  parse_str(
    r#"
      fn test(items []string) {
        for item in items {}
        for item in items {}
      }
    "#,
  );
}

#[test]
fn test_simple_for_names() {
  let tokens = parse_str(
    r#"
      fn test(items []string) {
        for item in items {}
      }
    "#,
  );
  match &tokens.functions["test"].body.actions[0].type_ {
    ActionType::For(res) => {
      if res.item_name != "item" {
        panic!("{:?}", tokens);
      }
      let list = &res.actions.actions;
      if list.len() != 0 {
        panic!("{:?}", tokens);
      }
      match &res.list.type_ {
        ActionType::VarRef(name) if name == "items" => {}
        _ => {
          panic!("{:?}", tokens);
        }
      }
    }
    _ => panic!("{:?}", tokens),
  }
}

#[test]
fn test_for_incorrect_args_1() {
  parse_str_fail(
    r#"
      fn test(items []string) {
        for a b c {}
      }
    "#,
  );
}

#[test]
fn test_for_incorrect_args_2() {
  parse_str_fail(
    r#"
      fn test(items []string) {
        for a in {}
      }
    "#,
  );
}

#[test]
fn test_for_incorrect_args_3() {
  parse_str_fail(
    r#"
      fn test(items []string) {
        for  in b {}
      }
    "#,
  );
}

#[test]
fn test_for_no_args() {
  parse_str_fail(
    r#"
      fn test(items []string) {
        for {}
      }
    "#,
  );
}

#[test]
fn test_simple_loop() {
  parse_str(
    r#"
      fn test(items []string) {
        loop {}
      }
    "#,
  );
}

#[test]
fn test_multiple_simple_loops() {
  parse_str(
    r#"
      fn test(items []string) {
        loop {}
        loop {}
      }
    "#,
  );
}

#[test]
fn test_loop_incorrect_args() {
  parse_str_fail(
    r#"
      fn test(items []string) {
        loop foo {}
      }
    "#,
  );
}

#[test]
fn test_simple_while() {
  parse_str(
    r#"
      fn test(items []string) {
        while true {}
      }
    "#,
  );
}

#[test]
fn test_multiple_simple_whiles() {
  parse_str(
    r#"
      fn test(items []string) {
        while true {}
        while true {}
      }
    "#,
  );
}

#[test]
fn test_incorrect_while_no_args() {
  parse_str_fail(
    r#"
      fn test(items []string) {
        while a b c {}
      }
    "#,
  );
}

#[test]
fn test_while_incorrect_args() {
  parse_str_fail(
    r#"
      fn test(items []string) {
        while a b c {}
      }
    "#,
  );
}

#[test]
fn test_break() {
  parse_str(
    r#"
      fn test() {
        loop {
          break
        }
      }
    "#,
  );
}

#[test]
fn test_continue() {
  parse_str(
    r#"
      fn test() {
        loop {
          continue
        }
      }
    "#,
  );
}

#[test]
fn test_break_outside_of_loop_fail() {
  parse_str_fail(
    r#"
      fn test() {
        break
      }
    "#,
  );
}

#[test]
fn test_continue_outside_of_loop_fail() {
  parse_str_fail(
    r#"
      fn test() {
        continue
      }
    "#,
  );
}

#[test]
fn test_loop_unreachable_code_warning_continue() {
  parse_str_warning(
    r#"
      fn test() {
        loop {
          continue
          test()
        }
      }
    "#,
  );
}

#[test]
fn test_loop_unreachable_code_warning_break() {
  parse_str_warning(
    r#"
      fn test() {
        loop {
          continue
          test()
        }
      }
    "#,
  );
}

#[test]
fn test_loop_unreachable_code_warning_return() {
  parse_str_warning(
    r#"
      fn test() {
        loop {
          return
          test()
        }
      }
    "#,
  );
}
