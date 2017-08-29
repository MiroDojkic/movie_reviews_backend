use diesel;
use diesel::prelude::*;
use diesel::result::QueryResult;
use schema::reviews;
use schema::reviews::dsl::*;

use establish_connection;
use models::review::{Review, NewReview};

pub fn get_all() -> QueryResult<Vec<Review>> {
    let connection = establish_connection();

    reviews.load::<Review>(&connection)
}

pub fn create(new_review: &NewReview) -> QueryResult<Review> {
    let connection = establish_connection();

    diesel::insert(new_review)
        .into(reviews::table)
        .get_result::<Review>(&connection)
}
