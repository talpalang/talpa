use super::*;

#[test]
fn simple_import() {
  parse_files(
    [
      (
        String::from("main.tp"),
        String::from(r#"import A "./a.tp""#),
      ),
      (String::from("a.tp"), String::new()),
    ]
    .iter()
    .cloned()
    .collect(),
  );
}

#[test]
fn multiple_import() {
  parse_files(
    [
      (
        String::from("main.tp"),
        String::from(
          r#"
            import
              A "./a.tp"
              B "b.tp"
              C "./c.tp"
          "#,
        ),
      ),
      (String::from("a.tp"), String::new()),
      (String::from("b.tp"), String::new()),
      (String::from("c.tp"), String::new()),
    ]
    .iter()
    .cloned()
    .collect(),
  );
}

#[test]
fn simple_import_with_something_after() {
  let after_options = vec![
    "fn abc() {}",
    "struct Abc{}",
    "enum Abc{}",
    "// abc",
    "const foo = \"1234\"",
  ];

  for after in after_options {
    parse_files(
      [
        (
          String::from("main.tp"),
          String::from("import A \"./a.tp\"\n") + after,
        ),
        (String::from("a.tp"), String::new()),
      ]
      .iter()
      .cloned()
      .collect(),
    );
  }
}
