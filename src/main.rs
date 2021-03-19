use facts::db;
use facts::lexer;

fn main() -> anyhow::Result<()> {
    let mut it = std::env::args();
    it.next();
    let query = it.collect::<Vec<_>>().join(" ");

    let db = db::open()?;

    let lexer = lexer::Lexer::new(&query);

    dbg!(db.hash(&query));

    let mut span: Option<(usize, usize)> = None;

    while let Some(t) = lexer.next()? {
        if matches!(t.kind, lexer::Kind::Word) {
            span = match span {
                Some((start, ..)) => Some((start, t.end)),
                None => Some((t.start, t.end)),
            };

            if let Some((start, end)) = span {
                let sentence = &query[start..end];

                if let Some(m) = db.lookup(sentence) {
                    match m {
                        db::Match::Constant(c) => {
                            dbg!(c);
                        }
                    }

                    span = None;
                }
            }

            continue;
        }

        dbg!(t);
    }

    Ok(())
}
