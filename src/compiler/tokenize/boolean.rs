use super::*;

#[derive(Debug, Clone)]
pub struct Boolean(pub bool);

impl Boolean {
  pub fn to_string(&self) -> String {
    if self.0 {
      "true".to_string()
    } else {
      "false".to_string()
    }
  }
}

impl Into<ActionType> for Boolean {
  fn into(self) -> ActionType {
    ActionType::StaticBoolean(self)
  }
}
