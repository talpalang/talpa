use super::CodeLocation;

static UPPER_CASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Checks if `name` is snake case
///
/// ```
/// // Valid
/// assert_eq!(true, is_snake_case("some_var_name"));
/// assert_eq!(true, is_snake_case("name"));
///
/// // Invalid
/// assert_eq!(false, is_pascal_case("Invalid"));
/// assert_eq!(false, is_pascal_case("alsoInvalid"));
/// assert_eq!(false, is_pascal_case("Mixed_Pascal_With_Snake"));
/// ```
pub fn is_snake_case(name: &str) -> bool {
  for c in name.chars() {
    if UPPER_CASE.contains(c) {
      return false;
    }
  }
  return true;
}

/// Checks if `name` is pascal case
///
/// ```
/// // Valid
/// assert_eq!(true, is_pascal_case("SomeVarName"));
/// assert_eq!(true, is_pascal_case("Name"));
///
/// // Invalid
/// assert_eq!(false, is_pascal_case("invalid"));
/// assert_eq!(false, is_pascal_case("snake_case_name"));
/// assert_eq!(false, is_pascal_case("Mixed_Pascal_With_Snake"));
/// ```
pub fn is_pascal_case(name: &str) -> bool {
  // Pascal case shout not contain underscores
  if name.contains('_') {
    return false;
  }

  // Check if the first char is upper case
  if let Some(first) = name.chars().next() {
    first.is_uppercase()
  } else {
    false
  }
}

pub trait GetName {
  fn name(&self) -> Option<String>;
}

pub trait GetLocation {
  fn location(&self) -> CodeLocation;
}
