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


    use routes::*;
    // Launch rocket!
    let _rocket = rocket::build()
        .manage(db)
        .mount("/", routes![
            account::create_account
        ])
        .launch()
        .await?;

    Ok(())
    
}