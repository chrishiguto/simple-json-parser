use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

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
struct Token<'a> {
    token_type: TokenType,
    value: &'a str,
}

fn main() {
    println!("Im running!");

    let file = match File::open("file.json") {
        Ok(file) => file,
        Err(e) => {
            println!("Error while reading the file. {}", e);
            std::process::exit(1);
        },
    };

    let reader = BufReader::new(file);

    let mut tokens: Vec<Token> = Vec::new();
    
    for line in reader.lines() {
        let read_line = match line {
            Ok(line) => line,
            Err(e) => {
                println!("Error while reading the line.");
                std::process::exit(1);
            },
        };
 
        let decimal_regex = Regex::new(r"[\d\w]").unwrap();
        let mut concatened_str = String::new();

        // TODO: this is probably inneficient
        for byte in read_line.chars() {
            match byte {
                '{' => tokens.push(Token {
                        token_type: TokenType::BraceOpen,
                        value: "{",
                    }),
                '}' => tokens.push(Token {
                        token_type: TokenType::BraceClose,
                        value: "}",
                    }),
                '[' => tokens.push(Token {
                        token_type: TokenType::BracketOpen,
                        value: "[",
                    }),
                ']' => tokens.push(Token {
                        token_type: TokenType::BracketClose,
                        value: "]",
                    }),
                ':' => tokens.push(Token {
                        token_type: TokenType::Colon,
                        value: ":",
                    }),
                ',' => tokens.push(Token {
                        token_type: TokenType::Comma,
                        value: ",",
                    }),
                '"' => {
                    if concatened_str.len() > 0 {
                        let new_str = String::from(concatened_str);

                        tokens.push(Token {
                            token_type: TokenType::String,
                            value: &new_str[..], 
                        });

                        concatened_str = String::new();
                    };

                    tokens.push(Token {
                        token_type: TokenType::Comma,
                        value: ",",
                    }) 
                },
                _ => {
                    if decimal_regex.is_match(&byte.to_string()[..]) {
                        concatened_str.push(byte);
                    }
                },
            }   
        }
    }

    println!("Tokens: {:?}", tokens);
}

