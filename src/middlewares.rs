extern crate movie_reviews_backend;
extern crate diesel;
extern crate serde_json;

use self::movie_reviews_backend::*;
use self::movie_reviews_backend::models::user::*;
use self::movie_reviews_backend::schema::users::dsl::*;
use self::diesel::prelude::*;

// Seems like Rust is still not OK with using extern macros,
// therefore cargo flags this as unused import,
// although it imports json macro
#[allow(unused_imports)]
use self::serde_json::*;

use iron::prelude::*;
use iron::status;

pub fn index(_: &mut Request) -> IronResult<Response> {

  let connection = establish_connection();
  let results = users.load::<User>(&connection).expect("Error loading users");
  println!("Displaying {} users", results.len());

  Ok(Response::with((status::Ok, json!(results).to_string())))
}
