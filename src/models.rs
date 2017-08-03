#![feature(rustc_private)]extern crate serialize;
#![feature(rustc_private)]extern crate rustc_serialize;
use serialize::json;

#[derive(Queryable, RustcDecodable, RustcEncodable)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub email: String,
}
