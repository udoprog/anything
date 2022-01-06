use std::io;

use anyhow::Result;
use codespan_reporting::term::termcolor::StandardStream;
use syntree::Span;
use syntree::{Nodes, Tree};

use crate::db;
use crate::error::Error;
use crate::eval::Context;
use crate::numeric::Numeric;
use crate::syntax::parser::{Parser, Syntax};

/// Description of things used by a query.
pub enum Description {
    /// A constant that was used.
    Constant(Box<str>, db::Constant),
}

/// The options of a query.
#[derive(Debug, Clone, Copy, Default)]
pub struct Options {
    pub(crate) describe: bool,
}

impl Options {
    /// Enable description of a query.
    pub fn describe(self) -> Self {
        Self {
            describe: true,
            ..self
        }
    }
}

/// A parsed node with associated source.
pub struct Parsed<'a> {
    source: &'a str,
    tree: Tree<Syntax>,
}

impl Parsed<'_> {
    /// Emit information on the current node.
    pub fn emit(&self, o: &mut StandardStream) -> Result<(), io::Error> {
        syntree::print::print_with_source(o, &self.tree, self.source)
    }
}

/// Parse the given source and return the corresponding node.
///
/// ```
/// use anything::parse;
///
/// let parsed = parse("0.99c");
/// ```
pub fn parse(source: &str) -> Result<Parsed<'_>> {
    let parser = Parser::new(source);
    let tree = parser.parse_root()?;
    Ok(Parsed { source, tree })
}

/// Perform a query over the given string and database.
///
/// ```
/// use anything::{Db, Options, parse, query};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let db = Db::open().unwrap();
/// let parsed = parse("0.99c")?;
///
/// let options = Options::default();
/// let mut descriptions = Vec::new();
/// let mut values = query(&parsed, &db, options, &mut descriptions);
///
/// assert!(matches!(values.next(), Some(Ok(..))));
/// # Ok(()) }
/// ```
pub fn query<'a>(
    parsed: &'a Parsed<'_>,
    db: &'a db::Db,
    options: Options,
    descriptions: &'a mut Vec<Description>,
) -> Query<'a> {
    Query {
        ctx: Context::new(),
        source: parsed.source,
        db,
        children: parsed.tree.children(),
        options,
        descriptions,
    }
}

/// The result of a query.
///
/// This can be iterator over to get results.
///
/// See [query].
pub struct Query<'a> {
    #[allow(unused)]
    pub(crate) ctx: Context,
    pub(crate) source: &'a str,
    pub(crate) db: &'a db::Db,
    pub(crate) children: Nodes<'a, Syntax>,
    pub(crate) options: Options,
    pub(crate) descriptions: &'a mut Vec<Description>,
}

impl<'a> Query<'a> {
    /// Get the current source as a string.
    pub(crate) fn source_as_str(&self) -> &'a str {
        self.source
    }

    /// Lookup the source corresponding to the given range.
    pub(crate) fn source(&self, span: Span) -> &'a str {
        &self.source[span.range()]
    }
}

impl Iterator for Query<'_> {
    type Item = Result<Numeric, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.children.next()?;
        Some(crate::eval::eval(self, node, Default::default()))
    }
}
