use super::*;

#[test]
fn test_simple_global_type_1() {
  parse_str(
    r#"
      type foo = string
    "#,
  );
}

#[test]
fn test_simple_global_type_2() {
  parse_str(
    r#"
      type foo = int
    "#,
  );
}

#[test]
fn test_multiple_simple_global_types() {
  parse_str(
    r#"
      type foo = string
      type bar = int
    "#,
  );
}

#[test]
fn test_advanced_global_type_1() {
  parse_str(
    r#"
      type foo = struct{}
    "#,
  );
}

#[test]
fn test_advanced_global_type_2() {
  parse_str(
    r#"
      type foo = []struct{}
    "#,
  );
}

#[test]
fn test_advanced_global_type_3() {
  parse_str(
    r#"
      type foo = []struct{
        bar string
        baz string
      }
    "#,
  );
}

#[test]
fn test_advanced_global_type_4() {
  parse_str(
    r#"
      type foo = [][]struct{
        bar string
        baz string
      }
    "#,
  );
}

#[test]
fn test_wired_global_type_1() {
  parse_str(
    r#"
      type foo =
      string
    "#,
  );
}

#[test]
fn test_wired_global_type_2() {
  parse_str(
    r#"
      type
        foo
          =
            string
    "#,
  );
}
