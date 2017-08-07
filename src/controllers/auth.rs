extern crate jsonwebtoken as jwt;

use super::bodyparser;
use iron::prelude::*;
use iron::status;
use iron::headers::{Authorization, Bearer};
use iron::error::HttpError;
use self::jwt::errors::Error;
use self::jwt::errors::ErrorKind::{InvalidToken, InvalidIssuer};

use models::auth::*;
use utils::auth::*;

pub fn login(req: &mut Request) -> IronResult<Response> {
    let user_login = req.get::<bodyparser::Struct<UserLogin>>();

    let user = match user_login {
        Ok(Some(user_login)) => Ok(match_user_credentials(&user_login)),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    };

    match user {
        Ok(Some(user)) => {
            match get_jwt(&user) {
                Ok(token) => Ok(Response::with((status::Ok, token.to_string()))),
                Err(e) => Err(IronError::new(e, status::InternalServerError)),
            }
        }
        Ok(None) => Ok(Response::with((status::NotFound, "Bad credentials!"))),
        Err(e) => Err(IronError::new(e, status::InternalServerError)),
    }
}

pub fn authenticate(req: &mut Request) -> IronResult<()> {
    let path = req.url.path().join("/");

    if path.starts_with("login") {
        return Ok(());
    }

    match req.headers.get::<Authorization<Bearer>>() {
        Some(authorization) => {
            match get_jwt_data(&authorization.token) {
                Ok() => Ok(()),
                Err(e) => Err(get_jwt_error_handler(e)),
            }
        }
        None => Err(IronError::new(
            HttpError::Header,
            status::InternalServerError,
        )),
    }
}

fn get_jwt_error_handler(error: Error) -> IronError {
    match *error.kind() {
        InvalidToken => IronError::new(error, status::Unauthorized),
        InvalidIssuer => IronError::new(error, status::Unauthorized),
        _ => IronError::new(error, status::InternalServerError),
    }
}
