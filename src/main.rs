use std::fs;
use std::iter::Peekable;

#[derive(Debug)]
enum TokenType {
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    String,
    Number,
    Comma,
    Colon,
    True,
    False,
    Null,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String,
}

#[derive(Debug)]
struct Parser<I: Iterator<Item = char>> {
    buffer: Peekable<I>,
    tokens: Vec<Token>,
    line: usize,
    col: usize,
}

// Parser should:
// 1. Tokenize
// 2. Parse tokens
impl<I: Iterator<Item = char>> Parser<I> {
    pub fn new(buffer: Peekable<I>) -> Self {
        Parser {
            buffer,
            tokens: Vec::new(),
            line: 1,
            col: 0,
        }
    }

    pub fn parse(&mut self) {
        self.tokenize();

        println!("Tokens: {:?}", self.tokens);
    }

    // The `tokenize` function should go through the `buffer` iterator
    // And build up the tokens.
    fn tokenize(&mut self) {
        // Every call to `self.next` will either return the char or None
        // Based on the char we'll add the corresponding token to the array of tokens.

        while let Some(char) = self.next() {
            match char {
                '{' => self.tokens.push(Token {
                    token_type: TokenType::BraceOpen,
                    value: "{".to_string(),
                }),
                '}' => self.tokens.push(Token {
                    token_type: TokenType::BraceOpen,
                    value: "}".to_string(),
                }),
                '[' => self.tokens.push(Token {
                    token_type: TokenType::BracketOpen,
                    value: "[".to_string(),
                }),
                ']' => self.tokens.push(Token {
                    token_type: TokenType::BracketClose,
                    value: "]".to_string(),
                }),
                '"' => {
                    self.tokens.push(Token {
                        token_type: TokenType::String,
                        value: "]".to_string(),
                    });

                    let mut concatened_char = String::new();
                    while let Some(char) = self.next() {
                        if char == '"' {
                            if concatened_char.len() > 0 {
                                self.tokens.push(Token {
                                    token_type: TokenType::String,
                                    value: concatened_char.clone(),
                                });
                            }

                            self.tokens.push(Token {
                                token_type: TokenType::String,
                                value: "]".to_string(),
                            });

                            break;
                        }

                        concatened_char.push(char)
                    }
                }
                _ => println!("Char: {}", char),
            }
        }
    }

    fn next(&mut self) -> Option<char> {
        while let Some(char) = self.buffer.next() {
            return Some(char);
        }

        None
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
