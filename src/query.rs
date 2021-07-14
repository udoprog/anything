use crate::db;
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
pub fn query<'a>(query: &'a str, db: &'a db::Db) -> Query<'a> {
    let parser = Parser::new(query);
    let node = parser.parse_root();

    Query {
        query,
        db,
        children: node.children(),
    }
}

pub struct Query<'a> {
    query: &'a str,
    db: &'a db::Db,
    children: rowan::SyntaxNodeChildren<FactsLang>,
}

impl Iterator for Query<'_> {
    type Item = anyhow::Result<Numeric>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.children.next()?;
        Some(crate::eval::eval(&self.query, node, self.db))
    }
}
