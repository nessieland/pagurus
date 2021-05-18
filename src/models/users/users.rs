use super::users_query::*;
use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(GraphQLObject, Queryable, Identifiable, PartialEq, Debug, Serialize, Deserialize)]
#[graphql(description = "A user object in Nessie")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub display_name: String,
    pub password: String,
    pub email: String,
    pub created_at: String,
}
