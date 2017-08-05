extern crate crypto;
extern crate diesel;

use ::{establish_connection};
use ::models::user::*;
use ::models::auth::*;
use schema::users::dsl::*;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use self::diesel::prelude::*;

pub fn hash(pwd: &str) -> String {
  let mut hasher = Sha256::new();

  hasher.input_str(pwd);
  hasher.result_str()
}

pub fn authenticate(user_login: &UserLogin) -> Option<User> {
  let connection = establish_connection();
  let user = users
    .filter(email.eq(&user_login.email))
    .get_result::<User>(&connection)
    .optional()
    .expect("Error loading users");

  if let Some(user) = user {
    let passhash = hash(&format!("{}{}", user_login.password, user.salt));

    if user.password == passhash {
      return Some(user);
    }
  };
  return None
}
