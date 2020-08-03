use super::*;

#[test]
fn test_variable() {
  parse_str(
    r#"
      const foo = "1234"
    "#,
  );
}

#[test]
fn test_variable_string_with_spaces() {
  parse_str(
    r#"
      const foo = "Hello world!"
    "#,
  );
}

#[test]
fn test_variable_strings_with_backslashes() {
  parse_str(
    r#"
      const foo = "I like to say \"Hello World!\" in my code."
      const bar = "This \\ backslash is displayed when printed!"
    "#,
  );
}

#[test]
fn test_multiple_variable_equal_names_fail() {
  parse_str_fail(
    r#"
      const foo = "bar"
      const foo = "bar"
    "#,
  );
}

#[test]
fn test_variable_global_let_fails() {
  parse_str_fail(
    r#"
      let foo = 0
    "#,
  );
}

#[test]
fn test_variable_starts_with_number_fails() {
  // variables should never start with a number
  parse_str_fail(
    r#"
      const 1fail = 0
    "#,
  );
}

#[test]
fn test_variable_invalid_name_warning() {
  // variables should never start with a number
  parse_str_warning(
    r#"
      const FooBar = 0
    "#,
  );
}
