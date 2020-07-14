use super::*;

#[test]
fn test_comment_single_line() {
  parse_str(
    r#"
      // hello world
    "#,
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
    "#,
  );
}

#[test]
fn test_comments_in_combination_with_functions() {
  parse_str(
    r#"
      fn foo() {}
      /*
        Multi-line comment.
        Can contain / and * and even /*
      */
      fn bar() {}
    "#,
  );
}

#[test]
fn test_comments_inside_of_function() {
  parse_str(
    r#"
      fn foo() {
        /*
          Multi-line comment.
          Can contain / and * and even /*
        */
      }
    "#,
  );
}
