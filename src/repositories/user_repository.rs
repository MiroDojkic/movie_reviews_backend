extern crate diesel;

use self::diesel::prelude::*;
use self::diesel::result::Error;
use schema::users::dsl::*;

use establish_connection;
use models::user::*;

pub fn get_all() -> Result<Vec<User>, Error> {
    let connection = establish_connection();

    users.load::<User>(&connection)
}

pub fn get_by_email(user_email: &String) -> Option<User> {
    let connection = establish_connection();

    users
        .filter(email.eq(user_email))
        .get_result::<User>(&connection)
        .optional()
        .expect("Error loading user by email.")
}
