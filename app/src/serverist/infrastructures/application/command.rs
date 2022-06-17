use diesel::Queryable;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub name: String,
    pub description: String,
    pub published: bool,
}