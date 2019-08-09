use diesel::r2d2::{ConnectionManager, Pool};

pub mod models;
pub mod schema;

pub fn create_pool<T: 'static + diesel::Connection>(
    connection: String,
) -> Pool<ConnectionManager<T>> {
    let manager = ConnectionManager::<T>::new(connection);
    Pool::new(manager).expect("Could not create database pool")
}
