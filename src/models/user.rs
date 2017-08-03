extern crate serde;
extern crate serde_json;
extern crate serde_derive;

use serde_json::Error;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub email: String,
}
