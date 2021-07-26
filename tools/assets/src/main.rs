use assets::db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let analyzer = assets::analyzer::Analyzer::new();
    let mut sources = db::Sources::new();

    {
        let mut db = db::Db::new();
        assets::astronomics::download(&mut db, &mut sources).await?;
        db::to_path("db/astronomics.bin.gz", &db)?;
    }

    {
        let mut db = db::Db::new();
        assets::populations::download(&analyzer, &mut db, &mut sources).await?;
        db::to_path("db/populations.bin.gz", &db)?;
    }

    {
        let mut db = db::Db::new();
        assets::copy_files::copy_files(&mut db).await?;
        db::to_path("db/files.bin.gz", &db)?;
    }

    db::to_path("db/sources.bin.gz", &sources)?;
    Ok(())
}
