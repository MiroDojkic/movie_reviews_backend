extern crate serde_json;
extern crate serde_derive;
extern crate bodyparser;
extern crate jsonwebtoken as jwt;

use ::{establish_connection};
use models::user::*;
use models::auth::*;
use models::auth::*;
use schema::users::dsl::*;

use utils::auth::*;

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

pub fn login(req: &mut Request) -> IronResult<Response> {
  let user_login = req.get::<bodyparser::Struct<UserLogin>>();

  let authenticated = match user_login {
    Ok(Some(user_login)) => Ok(authenticate(&user_login)),
    Ok(None) => Ok(None),
    Err(e) => Err(e)
  };

  Ok(Response::with((status::Ok, format!("{}", authenticated.unwrap().unwrap().email))))
}

pub fn index(_: &mut Request) -> IronResult<Response> {
  let connection = establish_connection();
  let results = users.load::<User>(&connection).expect("Error loading users");

  Ok(Response::with((status::Ok, json!(results).to_string())))
}
