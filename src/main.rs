extern crate iron;
extern crate router;
extern crate dotenv;
extern crate diesel;
extern crate jwt;
extern crate crypto;
extern crate movie_reviews_backend;

use self::movie_reviews_backend::*;
use std::env;

use dotenv::dotenv;
use iron::prelude::*;
use router::Router;
use diesel::migrations::{run_pending_migrations};

pub mod middlewares;

fn main() {
  dotenv().ok();

  let connection = establish_connection();
  match run_pending_migrations(&connection) {
    Ok(_) => println!("Migrations done"),
    Err(err) => println!("Migrations failed: {:?}", err),
  }

  let port = env::var("PORT").expect("PORT must be set");
  let host = format!("{}:{}", "0.0.0.0", port);
  let mut router = Router::new();

  router.get("/", middlewares::index, "index");
  router.post("/login", middlewares::authenticate, "authenticate");

  Iron::new(router).http(host).unwrap();
}
