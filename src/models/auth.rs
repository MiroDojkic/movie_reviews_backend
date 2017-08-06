extern crate serde;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    company: String
}

#[derive(Clone, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String
}
