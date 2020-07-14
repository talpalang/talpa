pub trait MatchString {
  /// The string that needs to be matched
  fn get_string(&self) -> &'static str;

  /// The char after the contents in get must be inside this matched to match the string obtained from get
  /// If returned None we don't match a character afterwards
  fn after(&self) -> Option<&'static str> {
    return None;
  }
}
