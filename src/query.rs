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

/// Perform a query over the given string and database.
///
/// ```rust
/// let db = facts::Db::open().unwrap();
/// let mut values = facts::query("0.99c", &db);
///
/// assert!(values.next().unwrap().is_ok());
/// ```
pub fn query<'a>(
    source: &'a str,
    db: &'a db::Db,
    options: Options,
    descriptions: &'a mut Vec<Description>,
) -> Query<'a> {
    let parser = Parser::new(source);
    let node = parser.parse_root();

    Query {
        ctx: Context::new(),
        source,
        db,
        children: node.children(),
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
    pub(crate) ctx: Context,
    pub(crate) source: &'a str,
    pub(crate) db: &'a db::Db,
    pub(crate) children: rowan::SyntaxNodeChildren<FactsLang>,
    pub(crate) options: Options,
    pub(crate) descriptions: &'a mut Vec<Description>,
}

impl Iterator for Query<'_> {
    type Item = Result<Numeric, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.children.next()?;
        Some(crate::eval::eval(self, node, Default::default()))
    }
}
