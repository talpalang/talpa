use super::*;

#[test]
fn test_simple_struct() {
  parse_str(
    r#"
      struct foo {}
    "#,
  );
}

#[test]
fn test_multiple_simple_structs() {
  parse_str(
    r#"
      struct foo {}
      struct bar {}
    "#,
  );
}

#[test]
fn test_invalid_inline_struct_in_global() {
  parse_str_fail(
    r#"
      struct {}
    "#,
  );
}

#[test]
fn test_struct_with_simple_field() {
  parse_str(
    r#"
      struct foo {
        bar string
      }
    "#,
  );
}

#[test]
fn test_struct_with_multiple_simple_fields() {
  parse_str(
    r#"
      struct foo {
        bar string
      }
    "#,
  );
}

#[test]
fn test_struct_with_inner_struct_1() {
  parse_str(
    r#"
      struct foo {
        bar struct {}
      }
    "#,
  );
}

#[test]
fn test_struct_with_inner_struct_2() {
  parse_str(
    r#"
      struct foo {
        bar struct {
          baz string
        }
      }
    "#,
  );
}

#[test]
fn test_struct_with_inner_struct_3() {
  parse_str(
    r#"
      struct foo {
        foo_bar string
        bar struct {
          baz struct {}
        }
        bar_foo string
      }
    "#,
  );
}

#[test]
fn test_invalid_struct_data() {
  parse_str_fail(
    r#"
      struct foo {
        bar
        baz
        foo_bar
      }
    "#,
  );
}
