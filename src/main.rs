extern crate iron;
extern crate router;
extern crate dotenv;
extern crate diesel;
extern crate common;

use common::{establish_connection, controllers};
use std::env;

use iron::prelude::*;
use router::Router;
use diesel::migrations::run_pending_migrations;

fn main() {
    let connection = establish_connection();
    match run_pending_migrations(&connection) {
        Ok(_) => println!("Migrations done"),
        Err(err) => println!("Migrations failed: {:?}", err),
    }

    let port = env::var("PORT").expect("PORT must be set");
    let host = format!("{}:{}", "0.0.0.0", port);
    let mut router = Router::new();

    router.get("/", controllers::user::index, "index");
    router.post(
        "/registration",
        controllers::user::registration,
        "registration",
    );
    router.post("/login", controllers::auth::login, "login");

    let mut chain = Chain::new(router);
    chain.link_before(controllers::auth::authenticate);

    Iron::new(chain).http(host).unwrap();
}
