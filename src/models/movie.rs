use super::Category;
use schema::{categories, movies};

#[derive(Debug, Clone, Deserialize, Identifiable, Queryable, Associations)]
#[belongs_to(Category)]
pub struct Movie {
    pub id: i32,
    pub name: String,
    pub director: String,
    pub year: i32,
    pub category_id: i32,
}
