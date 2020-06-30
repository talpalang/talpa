use super::*;

pub fn legal_name_char(c: char) -> bool {
  statics::VALID_NAME_CHARS.contains(c)
}
