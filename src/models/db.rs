use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use std::env;

use crate::models::entities::accounts::ActiveModel;


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


    pub fn get_by_id(self, id: u64) -> ActiveModel {

        todo!()

    }

    pub fn add(account: ActiveModel) -> core::result::Result<(), ()> {


        Ok(())
    }

    pub fn edit(account: ActiveModel) -> core::result::Result<(), ()> {


        Ok(())
    }

    pub fn delete(account: ActiveModel) -> core::result::Result<(), ()> {

        // Change deleted value to true
        Ok(())
    }

    pub fn get_id_by_username(username: String) -> u64 {

        32
    }


    pub fn get_id_by_email(email: String) -> u64 {

        32
    }


    pub fn get_members() -> Vec<u64> {


        vec![32]
    }

}