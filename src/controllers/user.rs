use super::bodyparser;
use super::rand::{thread_rng, Rng};
use iron::prelude::*;
use iron::status;
use iron::error::HttpError;

use models::user::{NewUser, UserRegistration};
use repositories::user_repository;

pub fn index(_: &mut Request) -> IronResult<Response> {
    let users = user_repository::get_all();

    Ok(Response::with(
        (status::Ok, json!(users.unwrap()).to_string()),
    ))
}

pub fn registration(req: &mut Request) -> IronResult<Response> {
    let new_user = req.get::<bodyparser::Struct<UserRegistration>>();

    match new_user {
        Ok(Some(parsed_user)) => {
            let salt = thread_rng().gen_ascii_chars().take(10).collect();


            let new_user_with_salt = NewUser {
                username: parsed_user.username.to_string(),
                email: parsed_user.email.to_string(),
                password: parsed_user.password.to_string(),
                salt: salt,
            };

            user_repository::create(&new_user_with_salt)
                .map_err(|e| IronError::new(e, status::InternalServerError))
                .and_then(|user| {
                    Ok(Response::with((status::Ok, json!(user).to_string())))
                })
        }
        Ok(None) | Err(_) => Err(IronError::new(HttpError::Header, status::BadRequest)),
    }
}
