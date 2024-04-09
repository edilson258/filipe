mod lexer;
mod token;

use lexer::Lexer;
use token::Token;

fn main() {
    let input = "print(\"Hello, World\")"
        .to_string()
        .chars()
        .collect::<Vec<char>>();

    let mut l = Lexer::new(&input);

    loop {
        let token = l.next_token();
        if let Token::Eof = token {
            println!("{:#?}", token);
            break;
        }
        println!("{:#?}", token);
    }
}
