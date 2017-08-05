extern crate serde_json;
extern crate serde_derive;
extern crate bodyparser;
extern crate jsonwebtoken as jwt;

use ::{establish_connection};
use common::models::user::*;
use common::schema::users::dsl::*;

use common::utils::auth::*;

use diesel::prelude::*;
use self::jwt::{encode, decode, Header, Validation};
use self::jwt::errors::{ErrorKind};

// Seems like Rust is still not OK with using extern macros,
// therefore cargo flags this as unused import,
// although it imports json macro
#[allow(unused_imports)]
use self::serde_json::*;
use self::serde_derive::*;

use iron::prelude::*;
use iron::status;

pub fn index(_: &mut Request) -> IronResult<Response> {
  let connection = establish_connection();
  let results = users.load::<User>(&connection).expect("Error loading users");

  Ok(Response::with((status::Ok, json!(results).to_string())))
}
