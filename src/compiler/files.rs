#[derive(Debug, Clone)]
pub struct CodeLocation {
  pub file_name: Option<String>,
  pub x: Option<usize>,
  pub y: Option<usize>,
}

impl CodeLocation {
  pub fn empty() -> Self {
    Self {
      file_name: None,
      x: None,
      y: None,
    }
  }
  pub fn only_file_name(file_name: String) -> Self {
    Self {
      file_name: Some(file_name),
      x: None,
      y: None,
    }
  }
  pub fn only_location(x: usize, y: usize) -> Self {
    Self {
      file_name: None,
      x: Some(x),
      y: Some(y),
    }
  }
}
