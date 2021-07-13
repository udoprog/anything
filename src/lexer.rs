use crate::parser::SyntaxKind;
use crate::span::Span;
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
    fn advance(&mut self) {
        let mut it = self.source[self.pos..].chars();

        self.pos = match it.next() {
            Some(c) => self.pos + c.len_utf8(),
            None => self.source.len(),
        };
    }

    fn consume_number(&mut self) -> usize {
        let mut count = 0;
        let mut dot = false;

        while let Some(c) = self.peek() {
            match c {
                '0'..='9' => {
                    self.advance();
                    count += 1;
                }
                '.' if !dot => {
                    self.advance();
                    dot = true;
                    count += 1;
                }
                'e' | 'E' => {
                    self.advance();
                    count += 1;

                    if let Some('-' | '+') = self.peek() {
                        self.advance();
                        count += 1;
                    }

                    while let Some('0'..='9') = self.peek() {
                        self.advance();
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

    fn consume_word(&mut self) {
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
    fn consume_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }

            self.advance();
        }
    }

    fn consume_unit_number(&mut self) {
        while let Some('0'..='9') = self.peek() {
            self.advance();
        }
    }

    fn consume_unit_word(&mut self) {
        while let Some('a'..='z' | 'A'..='Z') = self.peek() {
            self.advance();
        }
    }

    fn consume_escaped_unit_word(&mut self) -> bool {
        if !matches!(self.peek(), Some('{')) {
            return false;
        }

        self.advance();
        self.consume_unit_word();

        if !matches!(self.peek(), Some('}')) {
            return false;
        }

        self.advance();
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
                self.advance();
                self.consume_unit_number();
                UNIT_NUMBER
            }
            '{' => {
                if !self.consume_escaped_unit_word() {
                    ERROR
                } else {
                    UNIT_ESCAPED_WORD
                }
            }
            'a'..='z' | 'A'..='Z' => {
                self.advance();
                self.consume_unit_word();
                UNIT_WORD
            }
            '^' => {
                self.advance();
                CARET
            }
            '/' => {
                self.advance();
                SLASH
            }
            '*' => {
                self.advance();
                STAR
            }
            _ => return None,
        };

        Some(Token {
            span: Span::new(start, self.pos as u32),
            kind,
        })
    }

    /// Get the next token.
    pub fn next(&mut self) -> Option<Token> {
        if self.unit_mode {
            return self.next_unit();
        }

        let start = self.pos as u32;
        let c = self.peek()?;

        let kind = match c {
            c if self.ws_mode && c.is_whitespace() => {
                self.consume_whitespace();
                WHITESPACE
            }
            '.' => {
                self.consume_number();
                NUMBER
            }
            '0'..='9' => {
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
                self.advance();

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
        };

        Some(Token {
            span: Span::new(start, self.pos as u32),
            kind,
        })
    }
}
