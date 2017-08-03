extern crate iron;
extern crate router;
extern crate dotenv;

use std::env;

use dotenv::dotenv;

use iron::prelude::*;
use router::Router;

pub mod middlewares;

fn main() {
  dotenv().ok();

  let host = env::var("HOST").expect("HOST must be set");
  let port = env::var("PORT").expect("PORT must be set");
  let mut router = Router::new();

  router.get("/", middlewares::index, "index");

  Iron::new(router).http(format!("{}:{}", host, port)).unwrap();
}
