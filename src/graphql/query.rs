use super::context::Context;
use super::user::User;
use diesel::*;
use juniper::{FieldError, FieldResult, Value};

pub struct Query;

#[juniper::object(
  Context = Context
)]
impl Query {
    fn me(context: &Context) -> FieldResult<Option<User>> {
        use super::super::database::schema::users::dsl::*;
        match context.user_id {
            None => Ok(None),
            Some(user_id) => {
                let connection = context.database.get().map_err(|e| {
                    warn!(
                        "An error occurred while attempting to connect to database: {}",
                        e
                    );
                    FieldError::new("Unable to connect to database", Value::null())
                })?;
                let results = users
                    .filter(id.eq(user_id))
                    .load::<User>(&connection)
                    .map_err(|e| {
                        warn!("An error occurred while attempting to query Users: {}", e);
                        FieldError::new("Unable to connect to database", Value::null())
                    })?;
                if results.len() == 1 {
                    Ok(Some(results[0].clone()))
                } else {
                    Err(FieldError::new("Unknown user", Value::null()))
                }
            }
        }
    }
}
