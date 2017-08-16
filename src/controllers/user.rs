use super::bodyparser;

use iron::prelude::*;
use iron::status;

use models::user::{NewUser, UserRegistration};
use repositories::user_repository;
use utils::auth::{hash, get_salt};
use utils::validation::validate_email;

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
            if let Ok(false) = validate_email(parsed_user.email.to_string()) {
                return Ok(Response::with(
                    (status::BadRequest, "Not a valid email format."),
                ));
            };

            let salt = get_salt();
            let password_with_salt = format!("{}{}", parsed_user.password.to_string(), salt);

            let new_user_with_salt = NewUser {
                username: parsed_user.username.to_string(),
                email: parsed_user.email.to_string(),
                password: hash(&password_with_salt),
                salt: salt,
            };

            user_repository::create(&new_user_with_salt)
                .map_err(|e| IronError::new(e, status::InternalServerError))
                .and_then(|user| {
                    Ok(Response::with((status::Ok, json!(user).to_string())))
                })
        }
        Ok(None) => Ok(Response::with(
            (status::BadRequest, "Missing registration data."),
        )),
        Err(e) => Err(IronError::new(e, status::BadRequest)),
    }
}
