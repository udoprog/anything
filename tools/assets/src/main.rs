use assets::db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let analyzer = assets::analyzer::Analyzer::default();
    let mut sources = db::Sources::default();

    {
        let mut db = db::Db::default();
        assets::astronomics::download(&mut db, &mut sources).await?;
        db::to_path("db/astronomics.bin.gz", &db)?;
    }

    {
        let mut db = db::Db::default();
        assets::populations::download(&analyzer, &mut db, &mut sources).await?;
        db::to_path("db/populations.bin.gz", &db)?;
    }

    {
        let mut db = db::Db::default();
        assets::copy_files::copy_files(&mut db).await?;
        db::to_path("db/files.bin.gz", &db)?;
    }

    db::to_path("db/sources.bin.gz", &sources)?;
    Ok(())
}
