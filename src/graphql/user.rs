use super::context::Context;

pub use super::super::database::models::User;

#[juniper::object(
  Context = Context
)]
impl User {
    fn username(&self) -> String {
        self.username.clone()
    }
}
