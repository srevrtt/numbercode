#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Token {
  pub num: String
}

impl Token {
  pub fn new(num: String) -> Token {
    Self {
      num
    }
  }
}
