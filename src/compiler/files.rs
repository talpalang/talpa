#[derive(Clone)]
pub struct CodeLocation {
  pub file_name: Option<String>,
  pub x: Option<usize>,
  pub y: Option<usize>,
}
