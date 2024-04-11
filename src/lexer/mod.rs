use crate::token::Token;

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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.eof() {
            return Token::Eof;
        }

        let token = match self.curr_char {
            '(' => Some(Token::Lparen),
            ')' => Some(Token::Rparen),
            ',' => Some(Token::Comma),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Asterisk),
            '/' => Some(Token::Slash),
            ';' => Some(Token::Semicolon),
            '=' => Some(Token::Equal),
            '"' => Some(self.read_string()),
            _ => None,
        };

        if token.is_some() {
            self.read_char();
            return token.unwrap();
        }

        if self.curr_char.is_alphabetic() {
            return self.read_identifier();
        }

        if self.curr_char.is_numeric() {
            return self.read_number();
        }

        let illegal = Token::Illegal(self.curr_char);
        self.read_char();
        illegal
    }

    fn read_identifier(&mut self) -> Token {
        let literal = self.chop_while(|x| x.is_alphanumeric());
        // look for keywords
        match literal.as_str() {
            "let" => Token::Let,
            _ => Token::Identifier(literal),
        }
    }

    fn read_string(&mut self) -> Token {
        self.read_char();
        let literal = self.chop_while(|x| x != '"');
        return Token::String(literal);
    }

    fn read_number(&mut self) -> Token {
        let literal = self.chop_while(|x| x.is_numeric() || x == '.');
        return Token::Number(literal.parse::<f64>().unwrap());
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
}
