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
        // One line comments work !
        /*
          Multi-line comment.
          Can contain / and * and even /*
          or close the function
      }
        */
      }
    "#,
  );
}

#[test]
fn test_comment_direct_eof_1() {
  parse_str("//");
}

#[test]
fn test_comment_direct_eof_2() {
  parse_str(" //");
}

#[test]
fn test_comment_direct_eof_3() {
  parse_str("// ");
}

#[test]
fn test_multi_line_comment_direct_eof_1() {
  parse_str("/*");
}

#[test]
fn test_multi_line_comment_direct_eof_2() {
  parse_str(" /*");
}

#[test]
fn test_multi_line_comment_direct_eof_3() {
  parse_str("/* ");
}
