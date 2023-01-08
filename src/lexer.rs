use crate::token::Token;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Lexer {
  tkns: Vec<Token>
}

impl Lexer {
  pub fn new(src: String) -> Lexer {
    let mut tkns = vec![];
    let mut idx = 0;
    let mut current_num = String::new();

    while idx < src.len() - 1 {
      if !src.chars().nth(idx).unwrap().is_digit(10) && !src.chars().nth(idx).unwrap().is_whitespace() {
        panic!("Error: Only numbers allowed!");
      }

      current_num.push(src.chars().nth(idx).unwrap());

      if current_num.len() == 3 {
        tkns.push(Token::new(current_num));
        current_num = String::new();
      }

      idx += 1;
    }

    Self {
      tkns
    }
  }

  pub fn get_tkns(&self) -> Vec<Token> {
    self.tkns.clone()
  }
}
