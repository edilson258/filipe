use super::token::Token;

const NULL_CHAR: char = '\0';

pub struct Lexer<'a> {
    input: &'a [char],
    curr_char: char,
    pos: usize,
    read_pos: usize,
    line: usize,
    colm: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [char]) -> Self {
        let mut l = Lexer {
            input,
            curr_char: NULL_CHAR,
            pos: 0,
            read_pos: 0,
            line: 1,
            colm: 1,
        };

        l.read_char();

        l
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.pos = self.read_pos;
            self.curr_char = NULL_CHAR;
            return;
        }

        self.curr_char = self.input[self.read_pos];
        self.pos = self.read_pos;
        self.read_pos += 1;

        if self.curr_char == '\n' {
            self.line += 1;
            self.colm = 1;
        } else {
            self.colm += 1;
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        if self.eof() {
            return Ok(Token::Eof);
        }

        let token = match self.curr_char {
            '(' => Some(Token::Lparen),
            ')' => Some(Token::Rparen),
            '{' => Some(Token::Lbrace),
            '}' => Some(Token::Rbrace),
            '[' => Some(Token::Lbracket),
            ']' => Some(Token::Rbracket),
            ',' => Some(Token::Comma),
            '*' => Some(Token::Asterisk),
            '/' => Some(Token::Slash),
            ':' => Some(Token::Colon),
            '%' => Some(Token::Percet),
            '.' => Some(Token::Dot),
            '-' => {
                if self.next_char_is('-') {
                    self.read_char();
                    Some(Token::DoubleMinus)
                } else {
                    Some(Token::Minus)
                }
            }
            '+' => {
                if self.next_char_is('+') {
                    self.read_char();
                    Some(Token::DoublePlus)
                } else {
                    Some(Token::Plus)
                }
            }
            '!' => {
                if self.next_char_is('=') {
                    self.read_char();
                    Some(Token::NotEqual)
                } else {
                    Some(Token::Bang)
                }
            }
            '>' => {
                if self.next_char_is('=') {
                    self.read_char();
                    Some(Token::GratherOrEqual)
                } else {
                    Some(Token::GratherThan)
                }
            }
            '<' => {
                if self.next_char_is('=') {
                    self.read_char();
                    Some(Token::LessOrEqual)
                } else {
                    Some(Token::LessThan)
                }
            }
            '=' => {
                if self.next_char_is('=') {
                    self.read_char();
                    Some(Token::DoubleEqual)
                } else {
                    Some(Token::Equal)
                }
            }
            '"' => {
                let token = self.read_string();
                self.read_char();
                return token;
            }
            _ => None,
        };

        if token.is_some() {
            self.read_char();
            return Ok(token.unwrap());
        }

        if self.curr_char.is_alphabetic() {
            return Ok(self.read_identifier());
        }

        if self.curr_char.is_numeric() {
            return Ok(self.read_number());
        }

        let illegal = Token::Illegal(self.curr_char);
        self.read_char();
        Ok(illegal)
    }

    fn read_identifier(&mut self) -> Token {
        let literal = self.chop_while(|x| x.is_alphanumeric());
        // look for keywords
        match literal.as_str() {
            "let" => Token::Let,
            "define" => Token::Func,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            "string" => Token::TypeString,
            "int" => Token::TypeInt,
            "float" => Token::TypeFloat,
            "boolean" => Token::TypeBoolean,
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "in" => Token::In,
            "void" => Token::TypeVoid,
            "Array" => Token::ClassArray,
            "import" => Token::Import,
            _ => Token::Identifier(literal),
        }
    }

    fn read_string(&mut self) -> Result<Token, String> {
        self.read_char();
        let literal = self.chop_while(|x| x != '"');
        if self.curr_char != '"' {
            return Err(format!("Unbalanced '\"'"));
        }
        Ok(Token::String(literal))
    }

    fn read_number(&mut self) -> Token {
        let literal = self.chop_while(|x| x.is_numeric() || x == '.');
        if literal.contains(".") {
            return Token::Float(literal.parse::<f64>().unwrap());
        }
        return Token::Int(literal.parse::<i64>().unwrap());
    }

    fn skip_whitespace(&mut self) {
        self.chop_while(|x| x.is_whitespace());
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> String
    where
        P: FnMut(char) -> bool,
    {
        let start = self.pos;
        while !self.eof() && predicate(self.curr_char) {
            self.read_char();
        }
        self.chop(start, self.pos)
    }

    fn chop(&mut self, begin: usize, end: usize) -> String {
        self.input[begin..end].iter().collect::<String>()
    }

    fn eof(&mut self) -> bool {
        self.curr_char == NULL_CHAR
    }

    fn next_char_is(&mut self, x: char) -> bool {
        if self.read_pos >= self.input.len() {
            return false;
        }
        self.input[self.read_pos] == x
    }
}

#[cfg(test)]
mod tests {
    use super::super::token::Token;
    use super::Lexer;

    #[test]
    fn test_next_token() {
        let input = r#"            
let name = "Edilson"
let age: int = 20

define sum(x: int, y: int): int {
    let result = x + y
    return result
}

print(sum(34, 35))

if sum(5, 4) >= 10 {
    print("Never get printed")
} else {
    print("Will get printed")
}

for x in range(1, 10, 2) {
    if (x % 2 == 0) {
        print(x, " is even")
    }
}
"#;
        let expected_tokens = [
            Token::Let,
            Token::Identifier("name".to_string()),
            Token::Equal,
            Token::String("Edilson".to_string()),
            Token::Let,
            Token::Identifier("age".to_string()),
            Token::Colon,
            Token::TypeInt,
            Token::Equal,
            Token::Int(20),
            Token::Func,
            Token::Identifier("sum".to_string()),
            Token::Lparen,
            Token::Identifier("x".to_string()),
            Token::Colon,
            Token::TypeInt,
            Token::Comma,
            Token::Identifier("y".to_string()),
            Token::Colon,
            Token::TypeInt,
            Token::Rparen,
            Token::Colon,
            Token::TypeInt,
            Token::Lbrace,
            Token::Let,
            Token::Identifier("result".to_string()),
            Token::Equal,
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
            Token::Return,
            Token::Identifier("result".to_string()),
            Token::Rbrace,
            Token::Identifier("print".to_string()),
            Token::Lparen,
            Token::Identifier("sum".to_string()),
            Token::Lparen,
            Token::Int(34),
            Token::Comma,
            Token::Int(35),
            Token::Rparen,
            Token::Rparen,
            Token::If,
            Token::Identifier("sum".to_string()),
            Token::Lparen,
            Token::Int(5),
            Token::Comma,
            Token::Int(4),
            Token::Rparen,
            Token::GratherOrEqual,
            Token::Int(10),
            Token::Lbrace,
            Token::Identifier("print".to_string()),
            Token::Lparen,
            Token::String("Never get printed".to_string()),
            Token::Rparen,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Identifier("print".to_string()),
            Token::Lparen,
            Token::String("Will get printed".to_string()),
            Token::Rparen,
            Token::Rbrace,
            Token::For,
            Token::Identifier("x".to_string()),
            Token::In,
            Token::Identifier("range".to_string()),
            Token::Lparen,
            Token::Int(1),
            Token::Comma,
            Token::Int(10),
            Token::Comma,
            Token::Int(2),
            Token::Rparen,
            Token::Lbrace,
            Token::If,
            Token::Lparen,
            Token::Identifier("x".to_string()),
            Token::Percet,
            Token::Int(2),
            Token::DoubleEqual,
            Token::Int(0),
            Token::Rparen,
            Token::Lbrace,
            Token::Identifier("print".to_string()),
            Token::Lparen,
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::String(" is even".to_string()),
            Token::Rparen,
            Token::Rbrace,
            Token::Rbrace,
            Token::Eof,
        ];

        let input = input.chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(&input);
        for expected_token in expected_tokens {
            assert_eq!(expected_token, lexer.next_token().unwrap());
        }
    }
}
