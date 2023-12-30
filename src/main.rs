use std::error::Error;
use std::fs;
use std::iter::Peekable;

#[derive(Debug)]
enum Token {
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    String(String),
    Number(f64),
    Comma,
    Colon,
    Boolean(bool),
    Null,
}

#[derive(Debug)]
struct Parser<I: Iterator<Item = char>> {
    buffer: Peekable<I>,
    line: usize,
    col: usize,
}

impl<I: Iterator<Item = char>> Parser<I> {
    pub fn new(buffer: Peekable<I>) -> Self {
        Parser {
            buffer,
            line: 1,
            col: 0,
        }
    }

    pub fn parse(&mut self) {
        let tokens = self.lex();

        println!("Tokens: {:?}", tokens);
    }

    fn lex_string(&mut self) -> Option<Token> {
        let mut concatened_char = String::new();

        while let Some(char) = self.buffer.next() {
            if char == '"' {
                if concatened_char.len() > 0 {
                    break;
                }
            }

            concatened_char.push(char)
        }

        Some(Token::String(concatened_char))
    }

    fn lex_null(&mut self) -> Option<Token> {
        let concatened_char = self.lex_constant("null");

        match concatened_char {
            Some(&_) => Some(Token::Null),
            None => None,
        }
    }

    fn lex_number(&mut self) -> Option<Token> {
        let mut concatened_char = String::new();

        if let Some('-') = self.buffer.peek() {
            concatened_char.push('-');
            let _ = self.consume();
        }

        match self.buffer.peek() {
            Some('0'..='9') => {
                while let Some(number @ '0'..='9') = self.buffer.next() {
                    concatened_char.push(number);
                }
            }
            _ => (),
        }

        if let Some('.') = self.buffer.peek() {
            concatened_char.push('.');
            let _ = self.consume();

            match self.buffer.peek() {
                Some('0'..='9') => {
                    while let Some(number @ '0'..='9') = self.buffer.next() {
                        concatened_char.push(number);
                    }
                }
                _ => (),
            }
        }

        match concatened_char.parse() {
            Ok(num) => Some(Token::Number(num)),
            Err(_) => None,
        }
    }

    fn lex_true(&mut self) -> Option<Token> {
        let concatened_char = self.lex_constant("true");

        match concatened_char {
            Some(&_) => Some(Token::Boolean(true)),
            None => None,
        }
    }

    fn lex_false(&mut self) -> Option<Token> {
        let concatened_char = self.lex_constant("false");

        match concatened_char {
            Some(&_) => Some(Token::Boolean(false)),
            None => None,
        }
    }

    fn lex_constant(&mut self, constant: &'static str) -> Option<&str> {
        for char in constant.chars() {
            if let Some(buf_char) = self.buffer.next() {
                if char == buf_char {
                    continue;
                }

                return Some(constant);
            };
        }

        None
    }

    fn lex_object(&mut self) -> Option<Token> {
        match self.consume() {
            Ok('{') => Some(Token::BraceOpen),
            Ok('}') => Some(Token::BraceClose),
            _ => None,
        }
    }

    fn lex_array(&mut self) -> Option<Token> {
        match self.consume() {
            Ok('[') => Some(Token::BracketOpen),
            Ok(']') => Some(Token::BracketClose),
            _ => None,
        }
    }

    fn lex_delimiters(&mut self) -> Option<Token> {
        match self.consume() {
            Ok(':') => Some(Token::Colon),
            Ok(',') => Some(Token::Comma),
            _ => None,
        }
    }

    fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(char) = self.buffer.peek() {
            let token = match char {
                '{' | '}' => self.lex_object(),
                '[' | ']' => self.lex_array(),
                ',' | ':' => self.lex_delimiters(),
                '"' => self.lex_string(),
                '0'..='9' | '-' => self.lex_number(),
                't' => self.lex_true(),
                'f' => self.lex_false(),
                'n' => self.lex_null(),
                c => {
                    if c.is_whitespace() {
                        let _ = self.consume();
                        continue;
                    }

                    println!("Invalid character {c}");

                    if let Ok(_) = self.consume() {
                        continue;
                    }

                    None
                }
            };

            match token {
                Some(token) => tokens.push(token),
                None => (),
            };
        }

        tokens
    }

    fn consume(&mut self) -> Result<char, &str> {
        if let Some(char) = self.buffer.next() {
            return Ok(char);
        }

        Err("Unexpected error")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("file.json")?;

    let mut parser = Parser::new(file.chars().peekable());
    parser.parse();

    Ok(())
}
