use super::bodyparser;
use iron::prelude::*;
use iron::status;
use iron::headers::{Authorization, Bearer};
use iron::error::HttpError;

use models::auth::*;
use utils::auth::*;
use utils::url_helpers::includes_path;

pub fn login(req: &mut Request) -> IronResult<Response> {
    let user_login = req.get::<bodyparser::Struct<UserLogin>>();

    let user = user_login
        .map_err(|e| IronError::new(e, status::BadRequest))?
        .and_then(|user_login| match_user_credentials(&user_login));

    match user {
        Some(user) => {
            match get_jwt(&user) {
                Ok(token) => Ok(Response::with((status::Ok, token.to_string()))),
                Err(e) => Err(IronError::new(e, status::InternalServerError)),
            }
        }
        None => Ok(Response::with((status::NotFound, "Bad credentials!"))),
    }
}

// Used as before_middleware to intercept anonymous users
pub fn authenticate(req: &mut Request) -> IronResult<()> {
    let whitelisted = vec!["login", "registration"];
    if includes_path(whitelisted, &req.url).map_err(|e| {
        IronError::new(e, status::InternalServerError)
    })?
    {
        return Ok(());
    }

    match req.headers.get::<Authorization<Bearer>>() {
        Some(authorization) => {
            match get_jwt_data(&authorization.token) {
                Ok(_) => Ok(()),
                Err(e) => Err(IronError::new(e, status::Unauthorized)),
            }
        }
        None => Err(IronError::new(HttpError::Header, status::Unauthorized)),
    }
}
