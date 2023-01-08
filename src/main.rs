mod token;
mod lexer;
mod utils;

use lexer::Lexer;

fn main() {
  let src = utils::read_src("test.numc");
  let lex = Lexer::new(src);

  for tkn in lex.get_tkns() {
    println!("tkn: {}", tkn.num);
  }
}
