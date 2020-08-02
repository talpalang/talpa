use super::CodeLocation;

static UPPER_CASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub trait GetName {
  fn name(&self) -> Option<String>;
}

pub fn is_snake_case(name: &str) -> bool {
  for c in name.chars() {
    if UPPER_CASE.contains(c) {
      return false;
    }
  }
  return true;
}

/// A wrapper for is_snake_case
pub fn is_var_name(name: &str) -> bool {
  is_snake_case(name)
}

pub fn is_camel_case(name: &str) -> bool {
  !name.contains('_')
}

pub trait GetLocation {
  fn location(&self) -> CodeLocation;
}
