use chrono::FixedOffset;
use rocket::serde::json::Json;
use rocket::{http, State};
use sea_orm::{Set, ActiveModelTrait};
use serde::Deserialize;
use crate::models::{db::*, entities};

#[derive(Deserialize)] // Requester must send json matching this format
pub struct CreateAccountForm {
    username: String,
    hash: String,
}

/* CREATE ACCOUNT */
// Pass in the connection initialized in main.rs, and accept a JSON format with least-exposure info, return status to requester
#[post("/create-account", format = "json", data = "<create_account_form>")] // This DOES require requester to specify content-type as JSON
pub async fn create_account(db: &State<PGDatabase>, create_account_form: Json<CreateAccountForm>) -> http::Status {
    let account_data = create_account_form.into_inner();

    let account = entities::accounts::ActiveModel {
        username: Set(account_data.username),
        hash: Set(account_data.hash),
        salt: Set("uuuuuu".to_string()),
        account_start_date: Set( chrono::Local::now().naive_local() ),
        account_deleted: Set(false),
        membership_status: Set(false),
        active_membership_start_date: Set(None),
        active_membership_end_date: Set(None),
        first_membership_date: Set(None),
        pro_status: Set(false),
        .. Default::default() // Fill in others (id)
    };

    let db_result = account.save(&db.conn).await.unwrap(); // Takes the connection, which we have as part of the struct we're passing, in the "conn" field
    dbg!(db_result);

    http::Status::Ok
}


/* 
#[get("/account/<id>")]
pub async fn get_account(db: &State<PGDatabase>, id: u64) -> Json<Account> {

    let account: Account = db.get_by_id(id);
    
    // GET FROM DATABASE

    Json(account)
}

*/