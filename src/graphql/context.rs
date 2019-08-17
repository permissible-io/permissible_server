use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use uuid::Uuid;

pub struct Context {
    pub user_id: Option<i32>,
    pub database: Pool<ConnectionManager<SqliteConnection>>,
    pub request_uuid: Uuid,
}

impl juniper::Context for Context {}
