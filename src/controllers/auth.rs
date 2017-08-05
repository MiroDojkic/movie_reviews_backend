extern crate bodyparser;
extern crate iron;

use self::iron::prelude::*;
use self::iron::status;

use models::auth::*;
use utils::auth::*;

pub fn login(req: &mut Request) -> IronResult<Response> {
  let user_login = req.get::<bodyparser::Struct<UserLogin>>();

  let authenticated = match user_login {
    Ok(Some(user_login)) => Ok(authenticate(&user_login)),
    Ok(None) => Ok(None),
    Err(e) => Err(e)
  };

  Ok(Response::with((status::Ok, format!("{}", authenticated.unwrap().unwrap().email))))
}
