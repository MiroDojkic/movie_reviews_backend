extern crate crypto;
extern crate jsonwebtoken as jwt;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use self::jwt::{encode, decode, Header, Validation, TokenData};
use self::jwt::errors::Error;
use std::env;

use models::user::*;
use models::auth::*;
use repositories::user_repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    username: String,
}

pub fn hash(pwd: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.input_str(pwd);
    hasher.result_str()
}

pub fn match_user_credentials(user_login: &UserLogin) -> Option<User> {
    let user = user_repository::get_by_email(&user_login.email);

    if let Some(user) = user {
        let passhash = hash(&format!("{}{}", user_login.password, user.salt));

        if user.password == passhash {
            return Some(user);
        }
    };

    None
}

pub fn get_jwt(user: &User) -> Result<String, Error> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let claims = Claims {
        sub: user.email.to_string(),
        username: user.username.to_string(),
    };

    encode(&Header::default(), &claims, jwt_secret.as_ref())
}

pub fn get_jwt_data(jwt: String) -> Result<TokenData<Claims>, Error> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let validation = Validation { ..Validation::default() };

    match decode::<Claims>(&jwt, jwt_secret.as_ref(), &validation) {
        Ok(token_data) => Ok(token_data),
        Err(e) => Err(e)
    }
}
