use crate::model::Error;

pub struct Env {
    pub db_host: String,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub sud_reg_token: String
}

impl Env {
    pub fn new(db_host: String, db_name: String, db_user: String, db_password: String, sud_reg_token: String) -> Self {
        Self {
            db_host, db_name, db_user, db_password, sud_reg_token
        }
    }
}

pub async fn init_env() -> Result<Env, Error> {
    let db_host: String = env::var("DB_HOST").expect("DB_HOST not found in env variables");
    let db_name: String = env::var("DB_NAME").expect("DB_NAME not found in env variables");
    let db_user: String = env::var("DB_USER").expect("DB_USER not found in env variables");
    let db_password: String = env::var("DB_PASSWORD").expect("DB_PASSWORD not found in env variables");
    let sud_reg_token: String = env::var("SUD_REG_TOKEN").expect("SUD_REG_TOKEN not found in env variables");
    Ok(Env::new(&db_host, &db_name, &db_user, &db_password, &sud_reg_token))
}