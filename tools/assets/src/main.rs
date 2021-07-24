use assets::db::Db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut db = Db {
        constants: Vec::new(),
    };

    assets::planets::download(&mut db).await?;
    assets::satellites::download(&mut db).await?;

    db.write_to("db/astronomics.toml")?;
    Ok(())
}
