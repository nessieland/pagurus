use super::users::User;
use crate::db::establish_connection;
use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct UsersMutation;

fn time_since_epoch() -> u128 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).unwrap().as_millis()
}

#[derive(Insertable, Serialize, Deserialize, Queryable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    username: &'a str,
    display_name: &'a str,
    email: &'a str,
    password: &'a str,
    created_at: &'a str,
}

#[juniper::graphql_object]
impl UsersMutation {
    pub fn createUser(
        &self,
        _username: String,
        _display_name: String,
        _password: String,
        _email: String,
    ) -> User {
        use crate::schema::users;
        use crate::schema::users::dsl::*;

        let connection = establish_connection();
        let new_user = NewUser {
            username: &*_username,
            display_name: &*_display_name,
            password: &*_password,
            email: &*_email,
            created_at: &*time_since_epoch().to_string(),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(&connection)
            .expect("Error saying user")
    }
}
