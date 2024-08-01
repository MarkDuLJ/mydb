// SQL token definitions
use std::fmt::{self, Write};
use std::fmt::Display;
use std::str::Chars;
// use std::fmt::Write;

#[derive(PartialEq, Debug)]

pub(crate) enum Keyword {
    Create,
    Database,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Semicolon,
}

impl From<Keyword> for Token {
    fn from(keyword: Keyword) -> Self {
        Token::Keyword(keyword)
    }
}


pub struct Tokenizer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
}

impl <'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut tokenizer = Tokenizer {
            input: input.chars(),
            current_char: None,
        };

        tokenizer.advance();
        tokenizer
    }

    fn advance(&mut self){
        self.current_char = self.input.next();
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(current) = self.current_char {
            match current {
                ' ' | '\t' | '\n' | '\r' => {self.advance();}
                ';' => {
                    tokens.push(Token::Semicolon);
                    self.advance();
                }
                'A'..='Z' | 'a'..='z' => {
                    let identifier = self.collect_identifier();
                    match identifier.to_uppercase().as_str() {
                        "CREATE" => tokens.push(Keyword::Create.into()),
                        "DATABASE" => tokens.push(Keyword::Database.into()),
                        _ => tokens.push(Token::Identifier(identifier)),
                    }
                }
                _ => panic!("Unexpected char: {}", current),
            }
        }
        tokens
    }

    fn collect_identifier(&mut self) -> String{
        let mut identifier = String::new();
        while let Some(current) = self.current_char {
            if current.is_alphanumeric() || current == '_' {
                identifier.push(current);
                self.advance();
            }else {
                break;
            }
        }

        identifier
    }
}

