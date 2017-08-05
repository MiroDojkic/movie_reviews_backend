extern crate serde_json;
extern crate serde_derive;

use iron::prelude::*;
use iron::status;
use diesel::prelude::*;

// Seems like Rust is still not OK with using extern macros,
// therefore cargo flags this as unused import,
// although it imports json macro
#[allow(unused_imports)]
use self::serde_json::*;
use self::serde_derive::*;

use ::{establish_connection};
use models::user::*;
use schema::users::dsl::*;
use utils::auth::*;

pub fn index(_: &mut Request) -> IronResult<Response> {
  let connection = establish_connection();
  let results = users.load::<User>(&connection).expect("Error loading users");

  Ok(Response::with((status::Ok, json!(results).to_string())))
}
