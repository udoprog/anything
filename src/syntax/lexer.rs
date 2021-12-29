use crate::syntax::parser::SyntaxKind;
use crate::syntax::span::Span;
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
    pos: usize,
    escape: bool,
}

impl<'a> Lexer<'a> {
    /// Construct a new lexer.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            escape: false,
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
        self.source.get(self.pos..)?.chars().next()
    }

    /// Peek the next next character.
    fn peek2(&mut self) -> Option<char> {
        self.source.get(self.pos..)?.chars().nth(1)
    }

    /// Step to the next character.
    fn step(&mut self) {
        let mut it = self.source.get(self.pos..).into_iter().flat_map(str::chars);

        self.pos = match it.next() {
            Some(c) => self.pos + c.len_utf8(),
            None => self.source.len(),
        };
    }

    fn consume_number(&mut self, mut dot: bool) -> usize {
        let mut count = 0;

        while let Some(c) = self.peek() {
            match c {
                '0'..='9' => {
                    self.step();
                    count += 1;
                }
                '.' if !dot => {
                    self.step();
                    dot = true;
                    count += 1;
                }
                'e' | 'E' => {
                    if !matches!(self.peek2(), Some('-' | '+' | '0'..='9')) {
                        break;
                    }

                    self.step();
                    count += 1;

                    if let Some('-' | '+') = self.peek() {
                        self.step();
                        count += 1;
                    }

                    while let Some('0'..='9') = self.peek() {
                        self.step();
                        count += 1;
                    }
                }
                _ => {
                    break;
                }
            }
        }

        count
    }

    fn consume_word(&mut self) -> usize {
        let mut count = 0;

        while let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '°' | '\'') = self.peek() {
            count += 1;
            self.step();
        }

        count
    }

    fn consume_escaped_word(&mut self) -> usize {
        let mut count = 0;

        while let Some(c) = self.peek() {
            if c.is_whitespace() || c == '}' {
                break;
            }

            count += 1;
            self.step();
        }

        count
    }

    /// Consume until ws.
    fn consume_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }

            self.step();
        }
    }

    /// Next escaped sequence.
    fn next_escape(&mut self) -> Option<Token> {
        let start = self.pos;
        let c = self.peek()?;

        let kind = match c {
            c if c.is_whitespace() => {
                self.consume_whitespace();
                WHITESPACE
            }
            '}' => {
                self.step();
                self.escape = false;
                CLOSE_BRACE
            }
            _ => {
                if self.consume_escaped_word() > 0 {
                    WORD
                } else {
                    self.step();
                    ERROR
                }
            }
        };

        Some(Token {
            span: Span::new(start as u32, self.pos as u32),
            kind,
        })
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.escape {
            return self.next_escape();
        }

        let start = self.pos;
        let c = self.peek()?;

        let kind = match c {
            c if c.is_whitespace() => {
                self.consume_whitespace();
                WHITESPACE
            }
            '{' => {
                self.step();
                self.escape = true;
                OPEN_BRACE
            }
            '.' => {
                self.step();

                if self.consume_number(true) == 0 {
                    ERROR
                } else {
                    NUMBER
                }
            }
            ',' => {
                self.step();
                COMMA
            }
            '0'..='9' => {
                self.consume_number(false);
                NUMBER
            }
            '*' => {
                self.step();

                if matches!(self.peek(), Some('*')) {
                    self.step();
                    STARSTAR
                } else {
                    STAR
                }
            }
            '/' => {
                self.step();
                SLASH
            }
            '+' => {
                self.step();

                if self.consume_number(false) > 0 {
                    NUMBER
                } else {
                    PLUS
                }
            }
            '-' => {
                self.step();

                if self.consume_number(false) > 0 {
                    NUMBER
                } else {
                    DASH
                }
            }
            '^' => {
                self.step();
                CARET
            }
            '%' => {
                self.step();
                PERCENTAGE
            }
            '(' => {
                self.step();
                OPEN_PAREN
            }
            ')' => {
                self.step();
                CLOSE_PAREN
            }
            _ => {
                if self.consume_word() > 0 {
                    match &self.source[start..self.pos] {
                        "to" => TO,
                        _ => WORD,
                    }
                } else {
                    self.step();
                    ERROR
                }
            }
        };

        Some(Token {
            span: Span::new(start as u32, self.pos as u32),
            kind,
        })
    }
}
