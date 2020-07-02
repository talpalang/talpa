use super::*;

#[test]
fn test_comment_single_line() {
  parse_str(
    r#"
      // hello world
    "#
  );
}

#[test]
fn test_comment_multi_line() {
  parse_str(
    r#"
      /*
        Multi-line comment.
        Can contain / and * and even /*
      */
    "#
  );
}
