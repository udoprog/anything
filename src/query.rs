use std::collections::VecDeque;
use std::io;

use codespan_reporting::term::termcolor::StandardStream;
use rowan::{SyntaxNode, TextRange};

use crate::db;
use crate::error::Error;
use crate::eval::Context;
use crate::numeric::Numeric;
use crate::syntax::parser::{FactsLang, Parser};

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

pub struct Node<'a> {
    source: &'a str,
    node: SyntaxNode<FactsLang>,
}

impl Node<'_> {
    /// Emit information on the current node.
    pub fn emit(&self, o: &mut StandardStream) -> Result<(), io::Error> {
        use std::io::Write;

        writeln!(o, "- {:?}", self.node.kind())?;

        let mut queue = VecDeque::new();
        queue.extend(self.node.children_with_tokens().map(|c| (2usize, c)));

        while let Some((depth, n)) = queue.pop_front() {
            match n {
                rowan::NodeOrToken::Node(node) => {
                    writeln!(
                        o,
                        "{: >depth$}- {kind:?}",
                        "",
                        depth = depth,
                        kind = node.kind()
                    )?;
                    queue.extend(node.children_with_tokens().map(|c| (depth + 2, c)));
                }
                rowan::NodeOrToken::Token(tok) => {
                    writeln!(
                        o,
                        "{: >depth$}  {kind:?}: {source:?}",
                        "",
                        depth = depth,
                        kind = tok.kind(),
                        source = &self.source[tok.text_range()]
                    )?;
                }
            }
        }

        Ok(())
    }
}

/// Parse the given source and return the corresponding node.
pub fn parse(source: &str) -> Node<'_> {
    let parser = Parser::new(source);
    let node = parser.parse_root();
    Node { source, node }
}

/// Perform a query over the given string and database.
///
/// ```rust
/// let db = facts::Db::open().unwrap();
/// let mut values = facts::query("0.99c", &db);
///
/// assert!(values.next().unwrap().is_ok());
/// ```
pub fn query<'a>(
    node: Node<'a>,
    db: &'a db::Db,
    options: Options,
    descriptions: &'a mut Vec<Description>,
) -> Query<'a> {
    let children = node.node.children();

    Query {
        ctx: Context::new(),
        node,
        db,
        children,
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
    pub(crate) node: Node<'a>,
    pub(crate) db: &'a db::Db,
    pub(crate) children: rowan::SyntaxNodeChildren<FactsLang>,
    pub(crate) options: Options,
    pub(crate) descriptions: &'a mut Vec<Description>,
}

impl<'a> Query<'a> {
    /// Get the current source as a string.
    pub(crate) fn source_as_str(&self) -> &'a str {
        self.node.source
    }

    /// Lookup the source corresponding to the given range.
    pub(crate) fn source(&self, range: TextRange) -> &'a str {
        &self.node.source[range]
    }
}

impl Iterator for Query<'_> {
    type Item = Result<Numeric, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.children.next()?;
        Some(crate::eval::eval(self, node, Default::default()))
    }
}
