extern crate jsonwebtoken as jwt;
use super::bodyparser;
use iron::prelude::*;
use iron::status;
use self::jwt::errors::Error;
use self::jwt::errors::ErrorKind::{InvalidToken, InvalidIssuer};

use models::auth::*;
use utils::auth::*;

pub fn login(req: &mut Request) -> IronResult<Response> {
    let user_login = req.get::<bodyparser::Struct<UserLogin>>();

    let user = match user_login {
        Ok(Some(user_login)) => Ok(authenticate(&user_login)),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    };

    match user {
        Ok(Some(user)) => {
            match get_jwt(&user) {
                Ok(token) => Ok(Response::with((status::Ok, token.to_string()))),
                Err(error) => Err(get_jwt_error_handler(error)),
            }
        }
        Ok(None) => Ok(Response::with((status::NotFound, "Bad credentials!"))),
        Err(error) => Err(IronError::new(error, status::InternalServerError)),
    }
}

fn get_jwt_error_handler(error: Error) -> IronError {
    match *error.kind() {
        InvalidToken => IronError::new(error, status::Unauthorized),
        InvalidIssuer => IronError::new(error, status::Unauthorized),
        _ => IronError::new(error, status::InternalServerError)
    }
}
