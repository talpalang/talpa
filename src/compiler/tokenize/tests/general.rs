use super::*;

#[test]
fn test_empty_1() {
  parse_str(r#""#);
}

#[test]
fn test_empty_2() {
  parse_str(
    r#"



    "#,
  );
}
