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
    unit_mode: bool,
    ws_mode: bool,
}

impl<'a> Lexer<'a> {
    /// Construct a new lexer.
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            unit_mode: false,
            ws_mode: true,
        }
    }

    /// Get the text for the given span.
    pub fn text(&self, span: Span) -> &str {
        self.source
            .get(span.start()..span.end())
            .expect("expected span")
    }

    /// Set the lexer to run in unit mode, rewinding it to the given head if
    /// appropriate.
    pub fn set_mode(&mut self, head: Option<Token>, unit_mode: bool, ws_mode: bool) {
        if let Some(head) = head {
            self.pos = head.span.start();
        }

        self.unit_mode = unit_mode;
        self.ws_mode = ws_mode;
    }

    /// Peek the next character.
    fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    /// Advance to the next character.
    fn step(&mut self) {
        let mut it = self.source[self.pos..].chars();

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

        while let Some(c) = self.peek() {
            match c {
                c if c.is_alphanumeric() => {}
                '\'' => {}
                _ => break,
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

    fn consume_unit_number(&mut self) -> usize {
        let mut count = 0;

        while let Some('0'..='9') = self.peek() {
            count += 1;
            self.step();
        }

        count
    }

    fn consume_unit_word(&mut self) {
        while let Some('a'..='z' | 'A'..='Z' | '-') = self.peek() {
            self.step();
        }
    }

    fn consume_escaped_unit_word(&mut self) -> bool {
        self.consume_unit_word();

        if !matches!(self.peek(), Some('}')) {
            return false;
        }

        self.step();
        true
    }

    /// Run the lexer in unit mode.
    pub fn next_unit(&mut self) -> Option<Token> {
        let start = self.pos as u32;

        let kind = match self.peek()? {
            c if self.ws_mode && c.is_whitespace() => {
                self.consume_whitespace();
                WHITESPACE
            }
            '0'..='9' => {
                self.consume_unit_number();
                UNIT_NUMBER
            }
            '-' => {
                self.step();

                if self.consume_unit_number() == 0 {
                    ERROR
                } else {
                    UNIT_NUMBER
                }
            }
            '{' => {
                self.step();

                if self.consume_escaped_unit_word() {
                    UNIT_ESCAPED_WORD
                } else {
                    ERROR
                }
            }
            'a'..='z' | 'A'..='Z' => {
                self.step();
                self.consume_unit_word();
                UNIT_WORD
            }
            '^' => {
                self.step();
                CARET
            }
            '/' => {
                self.step();
                SLASH
            }
            '*' => {
                self.step();
                STAR
            }
            _ => {
                return None;
            }
        };

        Some(Token {
            span: Span::new(start, self.pos as u32),
            kind,
        })
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.unit_mode {
            return self.next_unit();
        }

        let start = self.pos;
        let c = self.peek()?;

        let kind = match c {
            c if self.ws_mode && c.is_whitespace() => {
                self.consume_whitespace();
                WHITESPACE
            }
            '.' => {
                self.step();

                if self.consume_number(true) == 0 {
                    ERROR
                } else {
                    NUMBER
                }
            }
            '0'..='9' => {
                self.consume_number(false);
                NUMBER
            }
            '*' => {
                self.step();
                STAR
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
                        "as" => AS,
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
