use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;

pub struct Context {
    pub user_id: Option<i32>,
    pub database: Pool<ConnectionManager<SqliteConnection>>,
}

impl juniper::Context for Context {}
