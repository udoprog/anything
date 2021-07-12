use anyhow::Result;
use facts::db;
use facts::parser;

fn main() -> Result<()> {
    let mut it = std::env::args();
    it.next();
    let query = it.collect::<Vec<_>>().join(" ");

    let db = db::open()?;

    let parser = parser::Parser::new(&query);
    let node = parser.parse_root();

    for expr in node.children() {
        let value = facts::eval::eval(&query, expr, &db)?;
        println!("{}", value);
    }

    Ok(())
}
