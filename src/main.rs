mod ast;
mod lexer;
mod token;
mod parser;

use parser::Parser;
use lexer::Lexer;

fn main() {
    let input = "3 * 5 + sum(4, 7)".to_string().chars().collect::<Vec<char>>();

    let mut l = Lexer::new(&input);
    let mut p = Parser::new(&mut l);
    let program = p.parse();
    if p.get_errors().len() > 0 {
        for e in p.get_errors() {
            println!("{}", e);
        }
    }
    println!("{:#?}, ", program);
}
