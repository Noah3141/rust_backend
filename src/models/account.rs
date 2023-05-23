/* 
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Account {

    username: String,
    email: String,
    password: String,

    account_start_date: Option<String>,
    account_deleted: bool,

    membership: bool,
    membership_start: Option<String>,
    membership_end: Option<String>,
    
    first_membership: Option<String>,

    pro: bool,

    id: u64
}

impl Account {

    pub fn new(username: String, email: String, password: String, account_start_date: Option<String>, account_deleted: bool, membership: bool, membership_start: Option<String>, membership_end: Option<String>, first_membership: Option<String>, pro: bool, id: u64
    ) -> Account {
        Account {
            username,
            email,
            password,
            account_start_date,
            account_deleted,
            membership,
            membership_start,
            membership_end,
            first_membership,
            pro,
            id
        }
    }

}


*/