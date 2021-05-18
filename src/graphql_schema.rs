use crate::models::users::{users::User, users_mutation::UsersMutation, users_query::UsersQuery};
use juniper::{EmptySubscription, RootNode};

#[derive(juniper::GraphQLObject)]
pub struct RootQuery {
    users: UsersQuery,
}

impl RootQuery {
    pub fn new() -> Self {
        RootQuery { users: UsersQuery }
    }
}

#[derive(juniper::GraphQLObject)]
pub struct RootMutation {
    users: UsersMutation,
}

impl RootMutation {
    pub fn new() -> Self {
        RootMutation {
            users: UsersMutation,
        }
    }
}

pub type Schema = RootNode<'static, RootQuery, RootMutation, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(
        RootQuery { users: UsersQuery },
        RootMutation {
            users: UsersMutation,
        },
        EmptySubscription::new(),
    )
}
