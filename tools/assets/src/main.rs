use assets::db::Db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let analyzer = assets::analyzer::Analyzer::new();

    {
        let mut db = Db::new();
        assets::planets::download(&mut db).await?;
        assets::satellites::download(&mut db).await?;
        db.to_path("db/astronomics.toml")?;
    }

    {
        let mut db = Db::new();
        assets::populations::download(&analyzer, &mut db).await?;
        db.to_path("db/populations.toml")?;
    }

    Ok(())
}
