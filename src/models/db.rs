use dotenvy::dotenv;
use sea_orm::{DatabaseConnection};
use std::env;

pub struct PGDatabase {
    pub conn: DatabaseConnection
}

impl PGDatabase {

    pub async fn conn() -> PGDatabase {
        dotenv().unwrap();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn  = sea_orm::Database::connect(&database_url).await.unwrap();
        PGDatabase {
            conn: conn
        }
    }
}
