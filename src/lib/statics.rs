pub static VALID_NAME_CHARS: &'static str =
  "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_";

pub fn legal_name_char(c: char) -> bool {
  VALID_NAME_CHARS.contains(c)
}

static CONST_KEYWORD: &'static str = "const";
static LET_KEYWORD: &'static str = "let";
static RETURN_KEYWORD: &'static str = "return";

#[derive(Clone, Copy)]
pub enum Keywords {
  Const,
  Let,
  Return,
}

impl Into<&'static str> for Keywords {
  fn into(self) -> &'static str {
    match self {
      Self::Const => CONST_KEYWORD,
      Self::Let => LET_KEYWORD,
      Self::Return => RETURN_KEYWORD,
    }
  }
}
