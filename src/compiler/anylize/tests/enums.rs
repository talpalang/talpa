use super::*;

#[test]
fn test_simple_enum() {
  parse_str(
    r#"
      enum foo {}
    "#,
  );
}

#[test]
fn test_invalid_enum_name() {
  parse_str_warning(
    r#"
      enum Foo {}
    "#,
  );
}

#[test]
fn test_multiple_simple_enums() {
  parse_str(
    r#"
      enum foo {}
      enum bar {}
    "#,
  );
}

#[test]
fn test_multiple_enums_equal_names_fail() {
  parse_str_fail(
    r#"
      enum foo {}
      enum foo {}
    "#,
  );
}

#[test]
fn test_invalid_inline_enum_in_global() {
  parse_str_fail(
    r#"
      enum {}
    "#,
  );
}

#[test]
fn test_enum_with_simple_field() {
  parse_str(
    r#"
      enum foo {
        bar
      }
    "#,
  );
}

#[test]
fn test_enum_with_multiple_simple_fields() {
  parse_str(
    r#"
      enum foo {
        bar
        baz
      }
    "#,
  );
}

#[test]
fn test_enum_with_multiple_fields_equal_names() {
  parse_str_fail(
    r#"
      enum foo {
        bar
        bar
      }
    "#,
  );
}

#[test]
fn test_enum_with_multiple_fields_invalid_names() {
  parse_str_warning(
    r#"
      enum foo {
        BarBaz
        BazBar
      }
    "#,
  );
}

#[test]
fn test_enum_with_field() {
  parse_str(
    r#"
      enum foo {
        bar = 1
      }
    "#,
  );
}

#[test]
fn test_enum_with_multiple_fields() {
  parse_str(
    r#"
      enum foo {
        bar = "bar"
        baz = "baz"
      }
    "#,
  );
}
