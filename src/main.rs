use tokenizer::{Token, Tokenizer};

fn main() {
    let source = "\
def fibonnaci(number):
  if number < 2:
    return 1
  return fibonnaci(number - 1) + fibonnaci(number - 2)
    ";
    let lexer = Tokenizer::new(source);
    print!("{:?}", lexer.collect::<Vec<Token>>());
}
