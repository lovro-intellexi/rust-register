use super::init_db;

#[tokio::test]
async fn db_init_db() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
    println!("{:?}", db);
    Ok(())
}