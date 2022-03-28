mod lexer;
mod token;

use lexer::Tokenizer;

fn main() -> () {
    Tokenizer::new("     let x").next_token();
}
