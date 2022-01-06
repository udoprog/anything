use crate::syntax::grammar;
use crate::syntax::lexer::{Lexer, Token};
use std::collections::VecDeque;
use syntree::{Id, Tree, TreeBuilder, TreeError};

/// Exact skip performed or to perform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Skip(usize);

impl Skip {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
}

/// The kind of a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Syntax {
    /// An expression whitespace.
    WHITESPACE,
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
    /// A unit.
    UNIT,

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
}

use Syntax::*;

/// A parser.
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    builder: TreeBuilder<Syntax>,
    buf: VecDeque<Token>,
}

impl<'a> Parser<'a> {
    /// Construct a new parser.
    pub fn new(source: &'a str) -> Parser<'a> {
        Self {
            lexer: Lexer::new(source),
            builder: TreeBuilder::new(),
            buf: VecDeque::new(),
        }
    }

    /// Consume and parse a root node.
    pub fn parse_root(mut self) -> Result<Tree<Syntax>, TreeError> {
        grammar::root(&mut self)?;
        Ok(self.builder.build()?)
    }

    /// Consume and parse a unit node.
    pub fn parse_unit(mut self) -> Result<Tree<Syntax>, TreeError> {
        if grammar::unit(&mut self, Skip::ZERO)?.is_none() {
            self.bump_empty_node(ERROR)?;
        }

        Ok(self.builder.build()?)
    }

    pub(crate) fn eat(&mut self, skip: Skip, expected: &[Syntax]) -> bool {
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

    /// Bump until the given syntax kind has been reached (or the whole parses
    /// has been consumed).
    pub(crate) fn bump_until(&mut self, kind: Syntax) {
        while let Some(node) = self.get(0) {
            self.bump();

            if node.kind == kind {
                break;
            }
        }
    }

    pub(crate) fn checkpoint(&mut self) -> Id {
        self.builder.checkpoint()
    }

    pub(crate) fn bump_empty_node(&mut self, kind: Syntax) -> Result<(), TreeError> {
        self.builder.open(kind);
        self.builder.close()?;
        Ok(())
    }

    pub(crate) fn bump_node(&mut self, kind: Syntax) -> Result<(), TreeError> {
        self.builder.open(kind);
        self.bump();
        self.builder.close()?;
        Ok(())
    }

    pub(crate) fn close_at(&mut self, c: Id, kind: Syntax) -> Result<(), TreeError> {
        self.builder.close_at(c, kind)?;
        Ok(())
    }

    pub(crate) fn error_node_at(&mut self, c: Id) -> Result<(), TreeError> {
        self.builder.close_at(c, ERROR)?;
        Ok(())
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

    pub(crate) fn nth(&mut self, skip: Skip, n: usize) -> Syntax {
        match self.get(skip.0 + n) {
            Some(t) => t.kind,
            None => EOF,
        }
    }

    pub(crate) fn bump(&mut self) {
        if let Some(t) = self.get(0) {
            self.builder.token(t.kind, t.len);
            self.buf.pop_front();
        }
    }

    pub fn count_skip(&mut self) -> Skip {
        let mut n = 0;

        while let Some(WHITESPACE) = self.get(n).map(|t| t.kind) {
            n += 1;
        }

        Skip(n)
    }
}
