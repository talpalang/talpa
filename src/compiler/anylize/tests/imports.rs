use super::*;

#[test]
fn simple_import() {
  parse_str(
    r#"
      import A "./a.tp"
    "#,
  );
}

#[test]
fn multiple_import() {
  parse_str(
    r#"
      import
        A "./a.tp"
        B "./b.tp"
        C "./c.tp"
    "#,
  );
}

#[test]
fn simple_import_with_function_after() {
  parse_str(
    r#"
      import A "./a.tp"

      fn abc() {}
    "#,
  );
}
