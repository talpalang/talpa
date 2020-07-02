use super::*;

#[test]
fn test_comment_single_line() {
  let res = parse_str(
    r#"
      // hello world
    "#
  );
}
