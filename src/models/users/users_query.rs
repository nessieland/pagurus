use super::users::User;
use crate::db::establish_connection;
use diesel::prelude::*;
use diesel::ExpressionMethods;

pub struct UsersQuery;

#[juniper::graphql_object]
impl UsersQuery {
    fn getUser(&self, _id: i32) -> User {
        use crate::schema::users;
        use crate::schema::users::dsl::*;

        let connection = establish_connection();

        users
            .filter(id.eq(_id))
            .limit(1)
            .load::<User>(&connection)
            .expect("Error fetching user")
    }
}
