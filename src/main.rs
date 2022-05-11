mod lexer;
mod token;

use lexer::Tokenizer;
use token::TokenKind::Eof;

fn main() -> () {
    let mut tokenizer = Tokenizer::new("let x = hello;");
    loop {
        let token = tokenizer.next_token();
        if token == Eof {
            break;
        }

        println!("{:?}", token);
    }
}
