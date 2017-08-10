extern crate regex;

use super::bodyparser;
use iron::prelude::*;
use iron::{status, Url};
use iron::headers::{Authorization, Bearer};
use iron::error::HttpError;
use self::regex::{Regex, Error};

use models::auth::*;
use utils::auth::*;

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
    if url_contains(&req.url, vec!["login", "registration"])
        .map_err(|e| IronError::new(e, status::BadRequest))?
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

fn url_contains(url: &Url, paths: Vec<&str>) -> Result<bool, Error> {
    let path = url.path().join("/");
    let pattern = paths.join("|");
    let regex = Regex::new(&pattern);

    regex.map(|re| re.is_match(&path))
}
