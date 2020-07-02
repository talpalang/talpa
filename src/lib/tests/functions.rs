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
fn test_functions_empty() {
  parse_str(
    r#"
                fn test1() {}
                fn test2() {}
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
fn test_function_with_args() {
  parse_str(
    r#"
                fn test(foo string, bar string, baz string) {}
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
                fn test(a int, b int) {}
                fn test_1() {
                    test(1, 2)
                }
            "#,
  );
}
