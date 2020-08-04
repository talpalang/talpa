use super::*;

#[test]
fn test_simple_global_type_1() {
  parse_str(
    r#"
      type Foo = string
    "#,
  );
}

#[test]
fn test_simple_global_type_2() {
  parse_str(
    r#"
      type Foo = int
    "#,
  );
}

#[test]
fn test_global_type_invalid_name_warning() {
  parse_str_warning(
    r#"
      type foo_bar = int
    "#,
  );
}

#[test]
fn test_simple_invalid_global_type_1() {
  parse_str_fail(
    r#"
      type foo =
    "#,
  );
}

#[test]
fn test_simple_invalid_global_type_2() {
  parse_str_fail(
    r#"
      type Foo
    "#,
  );
}

#[test]
fn test_multiple_simple_global_types() {
  parse_str(
    r#"
      type Foo = string
      type Bar = int
    "#,
  );
}

#[test]
fn test_multiple_global_types_equal_names_fail() {
  parse_str_fail(
    r#"
      type Foo = string
      type Foo = int
    "#,
  );
}

#[test]
fn test_advanced_global_type_1() {
  parse_str(
    r#"
      type Foo = struct{}
    "#,
  );
}

#[test]
fn test_advanced_global_type_2() {
  parse_str(
    r#"
      type Foo = []struct{}
    "#,
  );
}

#[test]
fn test_advanced_global_type_3() {
  parse_str(
    r#"
      type Foo = []struct{
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
      type Foo = [][]struct{
        bar string
        baz string
      }
    "#,
  );
}
