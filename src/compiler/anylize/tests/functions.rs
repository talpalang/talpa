use super::*;

#[test]
fn test_function_empty() {
  parse_str(
    r#"
      fn test() {}
    "#,
  );
}

#[test]
fn test_invalid_function_name() {
  parse_str_warning(
    r#"
      fn Test() {}
    "#,
  );
}

#[test]
fn test_functions_empty() {
  parse_str(
    r#"
      fn test1() {}
      fn test2() {}
    "#,
  );
}

#[test]
fn test_functions_empty_equal_name_fail() {
  parse_str_fail(
    r#"
      fn test() {}
      fn test() {}
    "#,
  );
}

#[test]
fn test_function_with_arg() {
  parse_str(
    r#"
      fn test(name string) {}
    "#,
  );
}

#[test]
fn test_function_with_arg_invalid_name_warning() {
  parse_str_warning(
    r#"
      fn test(Name string) {}
    "#,
  );
}

#[test]
fn test_function_with_args() {
  parse_str(
    r#"
      fn test(foo string, bar string, baz string) {}
    "#,
  );
}

#[test]
fn test_function_with_args_multiple_invalid_names_warnings() {
  parse_str_warning(
    r#"
      fn test(Foo string, Bar string, Baz string) {}
    "#,
  );
}

#[test]
fn test_function_with_args_multiple_equal_names_fail() {
  parse_str_fail(
    r#"
      fn test(foo string, foo int, foo uint) {}
    "#,
  );
}

#[test]
fn test_function_with_result() {
  parse_str(
    r#"
      fn test() string {
        return "a"
      }
    "#,
  );
}

#[test]
fn test_function_with_arg_and_result() {
  parse_str(
    r#"
      fn test(ab string) string {
        return ab
      }
    "#,
  );
}

#[test]
fn test_function_call_without_args() {
  parse_str(
    r#"
      fn test() {
      }
      fn main() {
        test()
      }
    "#,
  );
}

#[test]
fn test_function_call_with_args() {
  parse_str(
    r#"
      const a = 1
      const b = 2
      fn test(a int, b int) {}
      fn test_1() {
        test(a, b)
      }
    "#,
  );
}
