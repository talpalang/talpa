use super::*;

#[test]
fn test_comment_single_line() {
  parse_str(
    r#"
      // hello world
    "#
  );
}
