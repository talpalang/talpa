/// This is used in meany places to safe the location of code
///
/// Why the index and y?
/// There are a few things to consider when storing code:
/// 1. How memory efficent is it because of all tokens you will need a locaton to later report errors?
/// 2. How much memory / cpu power does it cost to know these locations?
/// 3. How easially can we get from this to debug information with code?
///
/// 1. This solution is only 64 + 16 bits per location so pretty memory efficent, we could go for only a index but that would have drawbacks in other points.
///
/// 2. For
#[derive(Debug, Clone)]
pub struct CodeLocation {
  pub index: usize,
  pub y: u16,
}

impl CodeLocation {
  pub fn new(index: usize, y: u16) -> Self {
    Self { index, y }
  }
}
