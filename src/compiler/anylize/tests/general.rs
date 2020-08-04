use super::*;

#[test]
fn test_empty() {
  parse_str(r#""#);
}

#[test]
fn fail_random_chars() {
  let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0987654321!@#$%^&*()_-=+~`{}[]:'\";:<>,.?/";

  // Only one char
  for c in chars.chars() {
    parse_str_fail_with_meta(c.to_string(), c.to_string());
  }

  // Multiple chars
  for a in chars.chars() {
    for b in chars.chars() {
      let to_parse = format!("{}{}", a, b);
      if &to_parse == "//" || &to_parse == "/*" {
        // These 2 char letters should pass
        parse_str(&to_parse);
      } else {
        parse_str_fail_with_meta(&to_parse, &to_parse);
      }
    }
  }
}
