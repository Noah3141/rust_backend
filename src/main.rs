#[macro_use] extern crate rocket; 
mod utils;
mod routes;
mod models;


use models::db::*;


#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    // Set up db struct as connected to db
    let db: PGDatabase = PGDatabase::conn().await;
    
    // Create table if not exists


    use routes::account::*;
    // Launch rocket!
    let _rocket = rocket::build()
        .manage(db)
        .mount("/", routes![
                // accounts
                create_account,
                update_account,
                get_account_by_id,
                get_account_by_username
        ])
        .launch()
        .await?;

    Ok(())
    
}