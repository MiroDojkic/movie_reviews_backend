extern crate iron;
extern crate router;
extern crate dotenv;
extern crate diesel;
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
  let success = run_pending_migrations(&connection);

  match success {
      Ok(_) => println!("Migrations passed"),
      Err(e) => println!("Error running migrations: {:?}", e),
  }

  let port = env::var("PORT").expect("PORT must be set");
  let host = format!("{}:{}", "0.0.0.0", port);
  let mut router = Router::new();

  router.get("/", middlewares::index, "index");

  Iron::new(router).http(host).unwrap();
}
