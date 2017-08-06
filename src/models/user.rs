extern crate serde;
extern crate serde_json;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub email: String,
  pub password: String,
  pub salt: String
}
