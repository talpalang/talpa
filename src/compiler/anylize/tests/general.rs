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
    parse_str_fail(c.to_string());
  }

  // Multiple chars
  for a in chars.chars() {
    for b in chars.chars() {
      parse_str_fail(format!("{}{}", a, b));
    }
  }
}
