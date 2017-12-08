use schema::categorys;

#[derive(Debug, Clone, Deserialize, Identifiable, Queryable)]
pub struct Category {
    pub id: i32,
    pub name: String,
}
