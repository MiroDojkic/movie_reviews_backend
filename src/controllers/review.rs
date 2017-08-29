use super::bodyparser;

use iron::prelude::*;
use iron::status;

use models::review::NewReview;
use repositories::review_repository;

pub fn all(_: &mut Request) -> IronResult<Response> {
    let reviews = review_repository::get_all();

    match reviews {
        Ok(reviews) => Ok(Response::with((status::Ok, json!(reviews).to_string()))),
        Err(e) => Err(IronError::new(e, status::InternalServerError)),
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
