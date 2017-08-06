extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

use ::models::user::*;
use ::models::auth::*;
use ::repositories::user_repository;

pub fn hash(pwd: &str) -> String {
  let mut hasher = Sha256::new();

  hasher.input_str(pwd);
  hasher.result_str()
}

pub fn authenticate(user_login: &UserLogin) -> Option<User> {
  let user = user_repository::get_by_email(&user_login.email);

  if let Some(user) = user {
    let passhash = hash(&format!("{}{}", user_login.password, user.salt));

    if user.password == passhash {
      return Some(user);
    }
  };
  return None
}
