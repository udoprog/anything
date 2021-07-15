fn main() -> anyhow::Result<()> {
    let mut it = std::env::args();
    it.next();
    let query = it.collect::<Vec<_>>().join(" ");

    let db = facts::Db::open()?;

    for value in facts::query(&query, &db) {
        match value {
            Ok(value) => {
                println!("{}", value);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}
