use std::collections::VecDeque;

use syntree::pointer::Width;
use syntree::{Builder, Checkpoint, Tree};

use crate::syntax::grammar;
use crate::syntax::lexer::{Lexer, Token};

type Result<T, E = syntree::Error> = std::result::Result<T, E>;

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
    builder: Builder<Syntax, u32, u32>,
    buf: VecDeque<Token>,
}

impl<'a> Parser<'a> {
    /// Construct a new parser.
    pub fn new(source: &'a str) -> Parser<'a> {
        Self {
            lexer: Lexer::new(source),
            builder: Builder::new_with(),
            buf: VecDeque::new(),
        }
    }

    /// Consume and parse a root node.
    pub fn parse_root(mut self) -> Result<Tree<Syntax, u32, u32>> {
        grammar::root(&mut self)?;
        self.builder.build()
    }

    /// Consume and parse a unit node.
    pub fn parse_unit(mut self) -> Result<Tree<Syntax, u32, u32>> {
        if grammar::unit(&mut self, Skip::ZERO)?.is_none() {
            self.bump_empty_node(ERROR)?;
        }

        self.builder.build()
    }

    pub(crate) fn eat(&mut self, skip: Skip, expected: &[Syntax]) -> Result<bool> {
        for (n, k) in expected.iter().enumerate() {
            match self.get(n) {
                Some(t) if t.kind == *k => {}
                _ => return Ok(false),
            }
        }

        for _ in 0..skip.0 {
            self.bump()?;
        }

        for _ in 0..expected.len() {
            self.bump()?;
        }

        Ok(true)
    }

    /// Bump until the given syntax kind has been reached (or the whole parses
    /// has been consumed).
    pub(crate) fn bump_until(&mut self, kind: Syntax) -> Result<()> {
        while let Some(node) = self.get(0) {
            self.bump()?;

            if node.kind == kind {
                break;
            }
        }

        Ok(())
    }

    pub(crate) fn checkpoint(&mut self) -> Result<Checkpoint<<u32 as Width>::Pointer>> {
        self.builder.checkpoint()
    }

    pub(crate) fn bump_empty_node(&mut self, kind: Syntax) -> Result<()> {
        self.builder.open(kind)?;
        self.builder.close()?;
        Ok(())
    }

    pub(crate) fn bump_node(&mut self, kind: Syntax) -> Result<()> {
        self.builder.open(kind)?;
        self.bump()?;
        self.builder.close()?;
        Ok(())
    }

    pub(crate) fn close_at(
        &mut self,
        c: &Checkpoint<<u32 as Width>::Pointer>,
        kind: Syntax,
    ) -> Result<()> {
        self.builder.close_at(c, kind)?;
        Ok(())
    }

    pub(crate) fn error_node_at(&mut self, c: &Checkpoint<<u32 as Width>::Pointer>) -> Result<()> {
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

    pub(crate) fn skip(&mut self, skip: Skip) -> Result<()> {
        for _ in 0..skip.0 {
            self.bump()?;
        }

        Ok(())
    }

    pub(crate) fn nth(&mut self, skip: Skip, n: usize) -> Syntax {
        match self.get(skip.0 + n) {
            Some(t) => t.kind,
            None => EOF,
        }
    }

    pub(crate) fn bump(&mut self) -> Result<()> {
        if let Some(t) = self.get(0) {
            self.builder.token(t.kind, t.len)?;
            self.buf.pop_front();
        }

        Ok(())
    }

    pub fn count_skip(&mut self) -> Skip {
        let mut n = 0;

        while let Some(WHITESPACE) = self.get(n).map(|t| t.kind) {
            n += 1;
        }

        Skip(n)
    }
}
