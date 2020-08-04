use super::*;

#[test]
fn test_simple_struct() {
  parse_str(
    r#"
      struct Foo {}
    "#,
  );
}

#[test]
fn test_struct_invalid_name_warning() {
  parse_str_warning(
    r#"
      struct foo {}
    "#,
  );
}

#[test]
fn test_multiple_simple_structs() {
  parse_str(
    r#"
      struct Foo {}
      struct Bar {}
    "#,
  );
}

#[test]
fn test_multiple_structs_equal_names_fail() {
  parse_str_fail(
    r#"
      struct Foo {}
      struct Foo {}
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
      struct Foo {
        bar string
      }
    "#,
  );
}

#[test]
fn test_struct_with_field_invalid_name_warning() {
  parse_str_warning(
    r#"
      struct Foo {
        BarBaz string
      }
    "#,
  );
}

#[test]
fn test_struct_with_multiple_simple_fields() {
  parse_str(
    r#"
      struct Foo {
        bar string
        baz string
      }
    "#,
  );
}

#[test]
fn test_struct_with_multiple_fields_equal_names_fail() {
  parse_str_fail(
    r#"
      struct Foo {
        bar string
        bar string
      }
    "#,
  );
}

#[test]
fn test_struct_with_inner_struct_1() {
  parse_str(
    r#"
      struct Foo {
        bar struct {}
      }
    "#,
  );
}

#[test]
fn test_struct_with_inner_struct_2() {
  parse_str(
    r#"
      struct Foo {
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
      struct Foo {
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
      struct Foo {
        bar
        baz
        foo_bar
      }
    "#,
  );
}
