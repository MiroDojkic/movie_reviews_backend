use super::bodyparser;
use iron::prelude::*;
use iron::status;

use models::user::NewUser;
use repositories::user_repository;

pub fn index(_: &mut Request) -> IronResult<Response> {
    let users = user_repository::get_all();

    Ok(Response::with(
        (status::Ok, json!(users.unwrap()).to_string()),
    ))
}

pub fn registration(req: &mut Request) -> IronResult<Response> {
    let new_user = req.get::<bodyparser::Struct<NewUser>>();

    Ok(Response::with((status::Ok, "OK!")))
}
