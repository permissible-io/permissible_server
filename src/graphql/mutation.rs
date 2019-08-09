use super::context::Context;

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    fn login() -> &'static str {
        "1"
    }
}
