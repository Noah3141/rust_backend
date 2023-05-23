use chrono::DateTime;
use rocket::serde::json::Json;
use rocket::{http, State};
use sea_orm::{Set, ActiveModelTrait};
use serde::Deserialize;
use crate::models::{
    account::Identifier, 
    entities::accounts, 
    {db::*, entities}
};


#[derive(Deserialize)] // Requester must send json matching this format
pub struct CreateAccountForm {
    pub username: String,
    pub hash: String,
}
/* CREATE ACCOUNT */
// Pass in the connection initialized in main.rs, and accept a JSON format with least-exposure info, return status to requester
#[post("/create-account", format = "json", data = "<create_account_form>")] // format = 'json' DOES require requester to specify content-type as JSON
pub async fn create_account(db: &State<PGDatabase>, create_account_form: Json<CreateAccountForm>) -> (http::Status, String) {
    let account_data = create_account_form.into_inner();
    match db.create_account(account_data).await {
        Ok(r) => (http::Status::Ok, String::from("Succesfully updated account")),
        Err(e) => (http::Status::NotModified, format!("Error updating account: {e}"))
    }
}

#[derive(Deserialize)] // Requester must send json matching this format
pub struct UpdateAccountForm {
    pub username: String,
    pub hash: String,
    pub account_deleted: bool,
    pub membership_status: bool, // As this gets updated, we want to change the relevant fields of the account with it
    pub pro_status: bool,
}

//  active_membership_start_date: Option<DateTime>,
//  active_membership_end_date: Option<DateTime>,
//  first_membership_date: Option<DateTime>,

#[post("/update-account", format = "json", data = "<update_account_form>")]
pub async fn update_account(db: &State<PGDatabase>, update_account_form: Json<UpdateAccountForm>) -> (http::Status, String) {
    let account_data = update_account_form.into_inner();
    match db.update_account(account_data).await {
        Ok(r) => (http::Status::Ok, String::from("Succesfully updated account")),
        Err(e) => (http::Status::NotModified, format!("Error updating account: {e}")),
    }
}


#[get("/get-account/by-id/<id>")]
pub async fn get_account_by_id(db: &State<PGDatabase>, id: i32) -> Result<Json<accounts::Model>, String> {
    let account = db.get_account(Identifier::Id(id)).await;
    match account {
        Ok(account) => match account {
            Some(account) => Ok(Json(account)),
            None => Err(String::from("Database returned None from your request"))
        }
        Err(e) => Err(String::from("Could not get the requested resource: {e}"))
    }   
}

#[get("/get-account/by-username/<username>")]
pub async fn get_account_by_username(db: &State<PGDatabase>, username: String) -> Result<Json<accounts::Model>, String> {
    let account = db.get_account(Identifier::Username(username)).await;
    match account {
        Ok(account) => match account {
            Some(account) => Ok(Json(account)),
            None => Err(String::from("Database returned None from your request"))
        }
        Err(e) => Err(String::from("Could not get the requested resource: {e}"))
    }   
}


