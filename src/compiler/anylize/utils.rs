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

pub fn is_camel_case(name: &str) -> bool {
  !name.contains('_')
}

pub trait GetLocation {
  fn location(&self) -> CodeLocation;
}
