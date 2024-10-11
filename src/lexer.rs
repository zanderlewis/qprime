use crate::token::Token;
use num_bigint::BigInt;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input[self.position];
        self.position += 1;

        match ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '>' => Token::GreaterThan,
            '<' => Token::LessThan,
            '=' => Token::Assign,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '"' => self.read_string_literal(),
            '0'..='9' => self.read_number(ch),
            'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(ch),
            ',' => Token::Comma,
            '#' => {
                while self.position < self.input.len() && self.input[self.position] != '\n' {
                    self.position += 1;
                }
                self.next_token()
            }
            _ => panic!("Unexpected character: {}", ch),
        }
    }

    pub fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    pub fn read_number(&mut self, first_char: char) -> Token {
        let mut number = first_char.to_string();
        while self.position < self.input.len() && self.input[self.position].is_digit(10) {
            number.push(self.input[self.position]);
            self.position += 1;
        }
        Token::Number(BigInt::parse_bytes(number.as_bytes(), 10).unwrap())
    }

    pub fn read_identifier(&mut self, first_char: char) -> Token {
        let mut identifier = first_char.to_string();
        while self.position < self.input.len() && (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_') {
            identifier.push(self.input[self.position]);
            self.position += 1;
        }
        match identifier.as_str() {
            "print" => Token::Print,
            "if" => Token::If,
            "else" => Token::Else,
            "dewpoint" => Token::DewPoint,
            "ftoc" => Token::FToC,
            "ctof" => Token::CToF,
            _ => Token::Identifier(identifier),
        }
    }

    pub fn read_string_literal(&mut self) -> Token {
        let mut string = String::new();
        while self.position < self.input.len() && self.input[self.position] != '"' {
            string.push(self.input[self.position]);
            self.position += 1;
        }
        self.position += 1; // Consume closing quote
        Token::StringLiteral(string)
    }
}