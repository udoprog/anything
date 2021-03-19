use std::cell::Cell;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unsupported character `{character}` at {pos}")]
    Unsupported { pos: usize, character: char },
}

/// The kind of a delimiter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delim {
    /// A parenthesis.
    Parenthesis,
}

/// The kind of a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// `*`.
    Star,
    /// `/`.
    Slash,
    /// `+`.
    Plus,
    /// `-`.
    Dash,
    /// `^`.
    Caret,
    /// Open delimiter.
    Open(Delim),
    /// Close delimiter.
    Close(Delim),
    /// A regular word.
    Word,
    /// A number.
    Number,
}

/// A lexed token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub kind: Kind,
}

/// The facts lexer.
pub struct Lexer<'a> {
    source: &'a str,
    pos: Cell<usize>,
}

impl<'a> Lexer<'a> {
    /// Construct a new lexer.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: Cell::new(0),
        }
    }

    /// Peek the next character.
    fn peek(&self) -> Option<char> {
        let n = self.pos.get();
        self.source[n..].chars().next()
    }

    /// Get the next character.
    fn advance(&self) -> Option<char> {
        let n = self.pos.get();
        let mut it = self.source[n..].char_indices();
        let c = it.next()?.1;
        let n = it.next().map(|v| n + v.0).unwrap_or(self.source.len());
        self.pos.set(n);
        Some(c)
    }

    /// Consume until ws.
    fn until_ws(&self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                break;
            }

            self.advance();
        }
    }

    /// The current lexer position.
    fn pos(&self) -> usize {
        self.pos.get()
    }

    /// Get the next token.
    pub fn next(&self) -> Result<Option<Token>, Error> {
        let mut start = self.pos();

        while let Some(c) = self.advance() {
            if c.is_whitespace() {
                start = self.pos();
                continue;
            }

            let kind = match c {
                '.' | '0'..='9' => {
                    self.until_ws();
                    Kind::Number
                }
                '*' => Kind::Star,
                '/' => Kind::Slash,
                '+' => Kind::Plus,
                '-' => Kind::Dash,
                '^' => Kind::Caret,
                '(' => Kind::Open(Delim::Parenthesis),
                ')' => Kind::Close(Delim::Parenthesis),
                _ => {
                    self.until_ws();
                    Kind::Word
                }
            };

            return Ok(Some(Token {
                start,
                end: self.pos(),
                kind,
            }));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::{Kind, Lexer, Token};

    #[test]
    fn test_lexer() {
        let source = "  42  ";
        let lexer = Lexer::new(source);

        let tok = lexer.next().unwrap().unwrap();

        assert!(matches!(
            tok,
            Token {
                kind: Kind::Number,
                ..
            }
        ));

        assert_eq!("42", &source[tok.start..tok.end]);
        assert!(lexer.next().unwrap().is_none());
    }
}
