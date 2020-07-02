pub static VALID_NAME_CHARS: &'static str =
  "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_";

pub fn legal_name_char(c: char) -> bool {
  VALID_NAME_CHARS.contains(c)
}

#[derive(Clone, Copy)]
pub enum Keywords {
  Const,
  Let,
  Return,
  Fn,
}

impl Into<&'static str> for Keywords {
  fn into(self) -> &'static str {
    match self {
      Self::Const => "const",
      Self::Let => "let",
      Self::Return => "return",
      Self::Fn => "fn",
    }
  }
}
