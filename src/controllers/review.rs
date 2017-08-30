use super::bodyparser;

use iron::prelude::*;
use iron::status;
use router::Router;
use models::review::NewReview;
use repositories::review_repository;

pub fn all(_: &mut Request) -> IronResult<Response> {
    let reviews = review_repository::get_all();

    match reviews {
        Ok(reviews) => Ok(Response::with((status::Ok, json!(reviews).to_string()))),
        Err(e) => Err(IronError::new(e, status::InternalServerError)),
    }
}

pub fn get_by_user(req: &mut Request) -> IronResult<Response> {
    let request_extensions = req.extensions.get::<Router>();
    match request_extensions {
        Some(request_extensions) => {
            let user_id = request_extensions.find("id");

            match user_id {
                Some(user_id) => {
                    let parsed_id = user_id.parse::<i32>();

                    match parsed_id {
                        Ok(parsed_id) => {
                            review_repository::get_by_user(parsed_id)
                                .map_err(|e| IronError::new(e, status::InternalServerError))
                                .and_then(|reviews| {
                                    Ok(Response::with((status::Ok, json!(reviews).to_string())))
                                })
                        }
                        Err(e) => Err(IronError::new(e, status::BadRequest)),
                    }
                }
                None => Ok(Response::with((status::BadRequest, "Missing user id!"))),
            }
        }
        None => Ok(Response::with((status::BadRequest, "Missing user id!"))),
    }
}

pub fn create(req: &mut Request) -> IronResult<Response> {
    let new_response = req.get::<bodyparser::Struct<NewReview>>();

    match new_response {
        Ok(Some(new_response)) => {
            review_repository::create(&new_response)
                .map_err(|e| IronError::new(e, status::InternalServerError))
                .and_then(|review| {
                    Ok(Response::with((status::Ok, json!(review).to_string())))
                })
        }
        Ok(None) => Ok(Response::with((status::BadRequest, "Missing review data!"))),
        Err(e) => Err(IronError::new(e, status::BadRequest)),
    }
}
