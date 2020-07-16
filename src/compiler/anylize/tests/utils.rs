use super::*;

#[test]
fn simple_valid_enums() {
  parse_str(
    r#"
      enum foo {}
      enum bar {}
    "#,
  )
}

#[test]
fn simple_invalid_enums() {
  parse_str_fail(
    r#"
      enum foo {}
      enum foo {}
    "#,
  )
}
