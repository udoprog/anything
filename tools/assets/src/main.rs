use assets::db::Db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let analyzer = assets::analyzer::Analyzer::new();

    {
        let mut db = Db::new();
        assets::planets::download(&mut db).await?;
        assets::satellites::download(&mut db).await?;
        db.to_path("db/astronomics.bin.gz")?;
    }

    {
        let mut db = Db::new();
        assets::populations::download(&analyzer, &mut db).await?;
        db.to_path("db/populations.bin.gz")?;
    }

    {
        let mut db = Db::new();
        assets::copy_files::copy_files(&mut db).await?;
        db.to_path("db/files.bin.gz")?;
    }

    Ok(())
}
