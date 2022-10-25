/*

// Insert subject into DB
let subject_to_save = Subject {
    _id: "123".to_string(),
    _rev: "".to_string(),
    oib: None,
    name: "Test".to_string(),
};
let mut value = to_value(subject_to_save)?;
db.create(&mut value).await?;

// Change subject data
let mut subject: Subject = db.get("123").await?;
subject.oib = Some("123456789".to_string());
db.save(&mut subject).await?;

// Get subject from DB
let _subject: Subject = db.get("123").await?;

println!("{:?}", _subject);


*/