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
    True,
    False,
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
            true => Some(Token::Null),
            false => None,
        }
    }

    fn lex_number(&mut self) -> Option<Token> {
        // Check for negative

        // Check for numbers

        // Check for dot

        let concatened_char = "10";

        match concatened_char.parse() {
            Ok(num) => Some(Token::Number(num)),
            Err(_) => None,
        }
    }

    fn lex_true(&mut self) -> Option<Token> {
        let concatened_char = self.lex_constant("true");

        match concatened_char {
            true => Some(Token::True),
            false => None,
        }
    }

    fn lex_false(&mut self) -> Option<Token> {
        let concatened_char = self.lex_constant("false");

        match concatened_char {
            true => Some(Token::False),
            false => None,
        }
    }

    fn lex_constant(&mut self, constant: &str) -> bool {
        for char in constant.chars() {
            if let Some(buf_char) = self.buffer.next() {
                if char == buf_char {
                    continue;
                }

                return false;
            };
        }

        true
    }

    fn lex_object(&mut self) -> Option<Token> {
        println!("Oi");

        match self.consume() {
            Ok('{') => Some(Token::BraceOpen),
            Ok('}') => Some(Token::BraceClose),
            _ => None,
        }
    }

    fn lex_array(&mut self) -> Option<Token> {
        println!("Oi");
        match self.consume() {
            Ok('[') => Some(Token::BracketOpen),
            Ok(']') => Some(Token::BraceClose),
            cj => {
                println!("Hey: {:?}", cj);

                None
            }
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
                // '0'..='9' | '-' => self.lex_number(),
                't' => self.lex_true(),
                'f' => self.lex_false(),
                'n' => self.lex_null(),
                c => {
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

fn main() {
    let file = match fs::read_to_string("file.json") {
        Ok(file) => file,
        Err(e) => {
            println!("Error while reading the file. {}", e);
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(file.chars().peekable());
    parser.parse();
}
