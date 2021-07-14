use crate::db;
use crate::error::Error;
use crate::numeric::Numeric;
use crate::parser::{FactsLang, Parser};

/// Perform a query over the given string and database.
///
/// ```rust
/// let db = facts::db::open().unwrap();
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

pub struct Query<'q, 'd> {
    query: &'q str,
    db: &'d db::Db,
    children: rowan::SyntaxNodeChildren<FactsLang>,
}

impl Iterator for Query<'_, '_> {
    type Item = Result<Numeric, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.children.next()?;
        Some(crate::eval::eval(
            node,
            &self.query,
            self.db,
            Default::default(),
        ))
    }
}
