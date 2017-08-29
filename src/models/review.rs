use super::user::User;

use schema::reviews;

#[derive(Debug, Clone, Serialize, Deserialize, Identifiable, Queryable, Associations)]
#[belongs_to(User, Movie)]
pub struct Review {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
    pub movie_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "reviews"]
pub struct NewReview {
    pub title: String,
    pub content: String,
    pub user_id: i32,
    pub movie_id: i32,
}
