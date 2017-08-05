extern crate bodyparser;
extern crate iron;
extern crate jsonwebtoken as jwt;

use self::iron::prelude::*;
use self::iron::status;
use self::jwt::{encode, decode, Header, Validation};
use self::jwt::errors::{ErrorKind};

use models::auth::*;
use utils::auth::*;

pub fn login(req: &mut Request) -> IronResult<Response> {
  let user_login = req.get::<bodyparser::Struct<UserLogin>>();

  let authenticated = match user_login {
    Ok(Some(user_login)) => Ok(authenticate(&user_login)),
    Ok(None) => Ok(None),
    Err(e) => Err(e)
  };

  match authenticated {
    Ok(Some(user)) => Ok(Response::with((status::Ok, format!("{}", user.email)))),
    Ok(None) => Ok(Response::with((status::NotFound, "Bad credentials!"))),
    Err(e) => Err(IronError::new(e, status::InternalServerError)),
  }
}
