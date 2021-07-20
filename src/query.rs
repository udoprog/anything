use crate::db;
use crate::error::Error;
use crate::eval::Context;
use crate::numeric::Numeric;
use crate::syntax::parser::{FactsLang, Parser};

/// Perform a query over the given string and database.
///
/// ```rust
/// let db = facts::Db::open().unwrap();
/// let mut values = facts::query("0.99c", &db);
///
/// assert!(values.next().unwrap().is_ok());
/// ```
pub fn query<'q, 'd>(query: &'q str, db: &'d db::Db) -> Query<'q, 'd> {
    let parser = Parser::new(query);
    let node = parser.parse_root();

    Query {
        query,
        db,
        children: node.children(),
    }
}

/// The result of a query.
///
/// This can be iterator over to get results.
///
/// See [query].
pub struct Query<'q, 'd> {
    query: &'q str,
    db: &'d db::Db,
    children: rowan::SyntaxNodeChildren<FactsLang>,
}

impl Iterator for Query<'_, '_> {
    type Item = Result<Numeric, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let ctx = Context::new();
        let node = self.children.next()?;

        Some(crate::eval::eval(
            &ctx,
            node,
            self.query,
            self.db,
            Default::default(),
        ))
    }
}
