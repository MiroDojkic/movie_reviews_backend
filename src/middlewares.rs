
extern crate movie_reviews_backend;
extern crate diesel;

use self::movie_reviews_backend::*;
use self::movie_reviews_backend::models::*;
use self::diesel::prelude::*;

use iron::prelude::*;
use iron::status;

pub fn index(req: &mut Request) -> IronResult<Response> {
  use self::movie_reviews_backend::schema::users::dsl::*;

  let connection = establish_connection();
  let results = users.load::<User>(&connection).expect("Error loading users");
  println!("Displaying {} users", results.len());

  Ok(Response::with((status::Ok, json::encode(&object).unwrap())))
}
