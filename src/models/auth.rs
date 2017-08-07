extern crate serde;
extern crate serde_json;

#[derive(Clone, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String
}
