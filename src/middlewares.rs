extern crate movie_reviews_backend;
extern crate diesel;
extern crate serde_json;
extern crate serde_derive;
extern crate bodyparser;

use self::movie_reviews_backend::*;
use self::movie_reviews_backend::models::user::*;
use self::movie_reviews_backend::schema::users::dsl::*;
use self::diesel::prelude::*;

// Seems like Rust is still not OK with using extern macros,
// therefore cargo flags this as unused import,
// although it imports json macro
#[allow(unused_imports)]
use self::serde_json::*;
use self::serde_derive::*;

use iron::prelude::*;
use iron::status;

#[derive(Clone, Deserialize)]
struct UserLogin {
    email: String,
    password: String
}

pub fn authenticate(req: &mut Request) -> IronResult<Response> {
  let user = req.get::<bodyparser::Struct<UserLogin>>().unwrap().unwrap();

  let user_email = user.email;
  let user_password = user.password;

  Ok(Response::with((status::Ok, user_email)))
}

pub fn index(_: &mut Request) -> IronResult<Response> {
  let connection = establish_connection();
  let results = users.load::<User>(&connection).expect("Error loading users");

  Ok(Response::with((status::Ok, json!(results).to_string())))
}
