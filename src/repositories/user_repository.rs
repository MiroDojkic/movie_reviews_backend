use diesel;
use diesel::prelude::*;
use diesel::result::QueryResult;
use schema::users;
use schema::users::dsl::*;

use establish_connection;
use models::user::*;

pub fn get_all() -> QueryResult<Vec<User>> {
    let connection = establish_connection();

    users.load::<User>(&connection)
}

pub fn get_by_email(user_email: &String) -> Option<User> {
    let connection = establish_connection();

    users
        .filter(email.eq(user_email))
        .get_result::<User>(&connection)
        .optional()
        .expect("Error loading user!")
}

pub fn create(new_user: &NewUser) -> QueryResult<User> {
    let connection = establish_connection();

    diesel::insert(new_user)
        .into(users::table)
        .get_result::<User>(&connection)
}
