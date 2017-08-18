use super::{User, Movie};
use schema::{reviews, users, movies};

#[derive(Debug, Clone, Deserialize, Identifiable, Queryable, Associations)]
#[belongs_to(User, Movie)]
pub struct Review {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
    pub movie_id: i32,
}
