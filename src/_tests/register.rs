#[tokio::test]
async fn register_getSubject() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
    let subject = Register::getSubject(&db, "123").await?;

    assert_eq!(subject.id, "123");
    println!("\n\n->> {:?}", subject);
    Ok(())
}