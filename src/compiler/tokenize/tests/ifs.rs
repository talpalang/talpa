use super::*;

#[test]
fn test_if() {
  parse_str(
    r#"
      fn test() {
        if true {}
      }
    "#,
  );
}

#[test]
fn test_wired_if() {
  parse_str(
    r#"
      fn test() {
        if
        true{    }
      }
    "#,
  );
}

#[test]
fn test_invalid_if() {
  parse_str_fail(
    r#"
      fn test() {
        if {}
      }
    "#,
  );
}

#[test]
fn test_if_else() {
  parse_str(
    r#"
      fn test() {
        if true {} else {}
      }
    "#,
  );
}

#[test]
fn test_wired_if_else() {
  parse_str(
    r#"
      fn test() {
        if
        true{    }
        else
        {

        }
      }
    "#,
  );
}

#[test]
fn test_invalid_if_else() {
  parse_str_fail(
    r#"
      fn test() {
        if true {} else
      }
    "#,
  );
}

#[test]
fn test_if_else_if() {
  parse_str(
    r#"
      fn test() {
        if true {} else if true {}
      }
    "#,
  );
}

#[test]
fn test_wired_if_else_if() {
  parse_str(
    r#"
      fn test() {
        if
        true{    }
        else
        if true{

        }
      }
    "#,
  );
}

#[test]
fn test_invalid_if_else_if() {
  parse_str_fail(
    r#"
      fn test() {
        if true {} else if {}
      }
    "#,
  );
}

#[test]
fn test_if_else_if_else() {
  parse_str(
    r#"
      fn test() {
        if true {} else if true {} else {}
      }
    "#,
  );
}

#[test]
fn test_wired_if_else_if_else() {
  parse_str(
    r#"
      fn test() {
        if
        true{    }
        else
        if true{

        }else{}
      }
    "#,
  );
}

#[test]
fn test_invalid_if_else_if_else() {
  parse_str_fail(
    r#"
      fn test() {
        if true {} else if true {} else
      }
    "#,
  );
}
