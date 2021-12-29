use crate::syntax::grammar;
use crate::syntax::lexer::{Lexer, Token};
use rowan::{Checkpoint, GreenNodeBuilder};
use std::collections::VecDeque;

/// Exact skip performed or to perform.
#[derive(Debug, Clone, Copy)]
pub struct Skip(usize);

impl Skip {
    pub const ZERO: Self = Self(0);
}

/// The kind of a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    /// An expression whitespace.
    WHITESPACE = 0,
    /// `*`.
    STAR,
    /// `**`.
    STARSTAR,
    /// `/`.
    SLASH,
    /// `+`.
    PLUS,
    /// `-`.
    DASH,
    /// `^`.
    CARET,
    /// `,`.
    COMMA,
    /// Open delimiter.
    OPEN_PAREN,
    /// Close delimiter.
    CLOSE_PAREN,
    /// Open brace.
    OPEN_BRACE,
    /// Close brace.
    CLOSE_BRACE,
    /// The `to` keyword.
    TO,
    /// A word.
    WORD,
    /// A sentence of words.
    SENTENCE,
    /// A number.
    NUMBER,
    /// A number with a unit.
    WITH_UNIT,

    /// The name of the function being called.
    FN_NAME,
    /// The arguments to the function being called.
    FN_ARGUMENTS,
    /// A function call.
    FN_CALL,

    /// A percentage expression.
    PERCENTAGE,

    /// Cast values.
    OP_CAST,
    /// Add values.
    OP_ADD,
    /// Subtract values.
    OP_SUB,
    /// Implicit multiplication.
    OP_IMPLICIT_MUL,
    /// Multiplication.
    OP_MUL,
    /// Divide values.
    OP_DIV,
    /// Power operation.
    OP_POWER,

    /// An operator in an operation.
    OPERATOR,
    /// An operation between two values.
    OPERATION,
    /// An error marker.
    ERROR,
    /// End of input.
    EOF,
    /// The root of an expression.
    ROOT,
}

use SyntaxKind::*;

/// A Facts syntax node.
pub type SyntaxNode = rowan::SyntaxNode<FactsLang>;
/// A Facts syntax token.
pub type SyntaxToken = rowan::SyntaxToken<FactsLang>;

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

/// A rowan language definition for Facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FactsLang {}

impl rowan::Language for FactsLang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::ROOT as u16);
        // Safety: we're asserting the layout above.
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

/// A parser.
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    builder: GreenNodeBuilder<'static>,
    buf: VecDeque<Token>,
}

impl<'a> Parser<'a> {
    /// Construct a new parser.
    pub fn new(source: &'a str) -> Parser<'a> {
        Self {
            lexer: Lexer::new(source),
            builder: GreenNodeBuilder::new(),
            buf: VecDeque::new(),
        }
    }

    /// Consume and parse a root node.
    pub fn parse_root(mut self) -> SyntaxNode {
        grammar::root(&mut self);
        SyntaxNode::new_root(self.builder.finish())
    }

    /// Consume and parse a unit node.
    pub fn parse_unit(mut self) -> SyntaxNode {
        if let EOF = self.nth(Skip::ZERO, 0) {
            self.builder.start_node(WORD.into());
            self.builder.finish_node();
        } else {
            let c = self.checkpoint();

            if !grammar::unit(&mut self) {
                self.bump();
                self.finish_node_at(c, ERROR);
            }
        }

        SyntaxNode::new_root(self.builder.finish())
    }

    pub(crate) fn eat(&mut self, skip: Skip, expected: &[SyntaxKind]) -> bool {
        for (n, k) in expected.iter().enumerate() {
            match self.get(n) {
                Some(t) if t.kind == *k => {}
                _ => return false,
            }
        }

        for _ in 0..skip.0 {
            self.bump();
        }

        for _ in 0..expected.len() {
            self.bump();
        }

        true
    }

    pub(crate) fn checkpoint(&mut self) -> Checkpoint {
        self.builder.checkpoint()
    }

    pub(crate) fn bump_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind.into());
        self.bump();
        self.builder.finish_node();
    }

    pub(crate) fn finish_node_at(&mut self, c: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(c, kind.into());
        self.builder.finish_node();
    }

    pub(crate) fn error_node_at(&mut self, c: Checkpoint) {
        self.builder.start_node_at(c, ERROR.into());
        self.builder.finish_node();
    }

    /// Fill the buffer up until the size of `n`.
    fn fill(&mut self, n: usize) {
        while self.buf.len() <= n {
            match self.lexer.next() {
                Some(t) => {
                    self.buf.push_back(t);
                }
                None => break,
            }
        }
    }

    fn get(&mut self, n: usize) -> Option<Token> {
        self.fill(n);
        self.buf.get(n).copied()
    }

    pub(crate) fn skip(&mut self, skip: Skip) {
        for _ in 0..skip.0 {
            self.bump();
        }
    }

    pub(crate) fn nth(&mut self, skip: Skip, n: usize) -> SyntaxKind {
        match self.get(skip.0 + n) {
            Some(t) => t.kind,
            None => EOF,
        }
    }

    pub(crate) fn bump(&mut self) {
        if let Some(t) = self.get(0) {
            let text = self.lexer.text(t.span);
            self.builder.token(t.kind.into(), text);
            self.buf.pop_front();
        }
    }

    pub(crate) fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind.into());
    }

    pub(crate) fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    pub fn count_skip(&mut self) -> Skip {
        let mut n = 0;

        while let Some(WHITESPACE) = self.get(n).map(|t| t.kind) {
            n += 1;
        }

        Skip(n)
    }
}
