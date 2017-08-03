extern crate iron;
extern crate router;

use std::net::{SocketAddrV4, Ipv4Addr};

use iron::prelude::*;
use router::Router;

pub mod middlewares;

fn main() {
    let mut router = Router::new();
    let host = SocketAddrV4::new(Ipv4Addr::new(0,0,0,0), 8080);

    router.get("/", middlewares::index, "index");

    Iron::new(router).http(host).unwrap();
}
