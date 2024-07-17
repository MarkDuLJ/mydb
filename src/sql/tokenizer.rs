use std::{fmt::Display, iter::Peekable, str::Chars};

use super::token::{Keyword, Token, Whitespace};

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) struct Location {
    // Line number, starting at 1,
    pub line: usize,
    // Column number, starting at 1.
    pub col:usize,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            line:1,
            col:1,
        }
    }
}

// store token and starting location
#[derive(Debug, PartialEq)]
pub(super) struct TokenWithLocation {
    pub variant: Token,
    pub location: Location,
}

impl TokenWithLocation {
    // remove locaiton, only keep token
    pub fn token_only(self) -> Token {
        self.variant
    }

    // reference to token
    pub fn token(&self) -> &Token {
        &self.variant
    }
}

// create a Stream struct to deal with input
struct Stream<'i> {
    input: &'i str,
    location: Location,
    chars: Peekable<Chars<'i>>,
}

impl<'i> Stream<'i> {
    fn new(input: &'i str) -> Self{
        Self {
            input,
            location: Location {line:1, col:1},
            chars: input.chars().peekable(),
        }
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next().inspect(|chr| {
            if *chr == '\n' {
                self.location.line += 1;
                self.location.col = 1;
            }else {
                {
                    self.location.col += 1;
                }
            }
        })
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn peek_next(&mut self) -> Option<&char>{
        self.next();
        self.peek()
    }

    fn locaiton(&self) -> Location {
        self.location
    }
}