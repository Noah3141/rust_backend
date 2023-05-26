use sea_orm::{
    Set, 
    ActiveModelTrait, 
    EntityTrait, 
    QueryFilter, 
    ColumnTrait, 
    DbErr
};
use crate::routes::account::{
    CreateAccountForm, 
    UpdateAccountForm
};
use super::{
    entities::accounts, 
    entities::accounts::{
        Entity as Account, 
        Column}
};

pub enum Identifier {
    Id(i32),
    Username(String)
}

impl super::db::PGDatabase {

    pub async fn create_account(&self, create_account_form: CreateAccountForm) -> Result<(), DbErr> {
        let account = accounts::ActiveModel {
            username: Set(create_account_form.username),
            hash: Set(create_account_form.hash),
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
        let db_result = account
            .save(&self.conn)
            .await?; // Takes the connection, which we have as part of the struct we're passing, in the "conn" field
        Ok(())
    }

    pub async fn get_account(&self, identifier: Identifier) -> Result<Option<accounts::Model>, DbErr > {
        let account = match identifier {
            Identifier::Id(id) => {
                Account::find_by_id(id)
                    .one(&self.conn)
                    .await
            },
            Identifier::Username(username) => {
                Account::find()
                    .filter(Column::Username.eq(username))
                    .one(&self.conn)
                    .await
            },
        };
        account
    }

    pub async fn update_account(&self, update_form: UpdateAccountForm) -> Result<(), DbErr> {

        todo!()
    }




}