use diesel::Queryable;

#[derive(Clone, Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
}
