use crate::parser::SyntaxKind;
use crate::span::Span;
use std::cell::Cell;
use SyntaxKind::*;

/// A lexed token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub span: Span,
    pub kind: SyntaxKind,
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

    /// Get the text for the given span.
    pub fn text(&self, span: Span) -> &str {
        self.source
            .get(span.start()..span.end())
            .expect("expected span")
    }

    /// Peek the next character.
    fn peek(&self) -> Option<char> {
        let n = self.pos.get();
        self.source[n..].chars().next()
    }

    /// Advance to the next character.
    fn advance(&self) {
        let n = self.pos.get();
        let mut it = self.source[n..].chars();

        let n = match it.next() {
            Some(c) => n + c.len_utf8(),
            None => self.source.len(),
        };

        self.pos.set(n);
    }

    fn consume_number(&self) -> usize {
        let mut dot = false;
        let mut count = 0;

        if let Some('-' | '+') = self.peek() {
            self.advance();
            count += 1;
        }

        while let Some(c) = self.peek() {
            match c {
                '0'..='9' => {
                    self.advance();
                }
                '.' if !dot => {
                    self.advance();
                    dot = true;
                }
                'e' | 'E' => {
                    self.advance();

                    if let Some('-' | '+') = self.peek() {
                        self.advance();
                    }

                    while let Some('0'..='9') = self.peek() {
                        self.advance();
                    }
                }
                _ => {
                    break;
                }
            }
        }

        count
    }

    fn consume_word(&self) {
        while let Some(c) = self.peek() {
            match c {
                c if c.is_alphanumeric() => {}
                '\'' => {}
                _ => break,
            }

            self.advance();
        }
    }

    /// Consume until ws.
    fn consume_whitespace(&self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
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
    pub fn next(&self) -> Option<Token> {
        let start = self.pos() as u32;
        let c = self.peek()?;

        let kind = if c.is_whitespace() {
            self.consume_whitespace();
            WHITESPACE
        } else {
            match c {
                '.' | '0'..='9' => {
                    self.consume_number();
                    NUMBER
                }
                '*' => {
                    self.advance();
                    STAR
                }
                '/' => {
                    self.advance();
                    SLASH
                }
                '+' => {
                    self.advance();

                    if self.consume_number() > 0 {
                        NUMBER
                    } else {
                        PLUS
                    }
                }
                '-' => {
                    if self.consume_number() > 0 {
                        NUMBER
                    } else {
                        DASH
                    }
                }
                '^' => {
                    self.advance();
                    CARET
                }
                '%' => {
                    self.advance();
                    PERCENTAGE
                }
                '(' => {
                    self.advance();
                    OPEN_PAREN
                }
                ')' => {
                    self.advance();
                    CLOSE_PAREN
                }
                _ => {
                    self.consume_word();
                    WORD
                }
            }
        };

        Some(Token {
            span: Span::new(start, self.pos() as u32),
            kind,
        })
    }
}
