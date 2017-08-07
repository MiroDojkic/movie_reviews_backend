use iron::prelude::*;
use iron::status;

use repositories::user_repository;

pub fn index(_: &mut Request) -> IronResult<Response> {
    let users = user_repository::get_all();

    Ok(Response::with(
        (status::Ok, json!(users.unwrap()).to_string()),
    ))
}
