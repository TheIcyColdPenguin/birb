mod lexer;
mod token;

use lexer::Tokenizer;
use token::TokenKind::Eof;

fn main() -> () {
    let mut tokenizer = Tokenizer::new(
        r#"let x = "huh";
print(x);


let x = 1;
let y = 2.1;

if x > y {
    print("\"{x}\" is greater than \"{y}\"");
} elif x == y {
    print("'{x}' is equal to \"{y}\""); 
} else {
    print('\'{x}\' is less than "{y}" 🥲')
}


func fact(int n) {
    if n <= 1 { n } else { n * fact(n-1) }
}
"#,
    );
    loop {
        let token = tokenizer.next_token();
        if token == Eof {
            break;
        }

        println!("{:?}", token);
    }
}
