use dotenv::dotenv;
use futures::future;
use hyper::header::AUTHORIZATION;
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::Method;
use hyper::{Body, Response, Server, StatusCode};
use juniper::RootNode;
use std::env;
use std::sync::Arc;

use permissible_server::database::create_pool;
use permissible_server::graphql::context::Context;
use permissible_server::graphql::mutation::Mutation;
use permissible_server::graphql::query::Query;

fn main() {
    pretty_env_logger::init();
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = create_pool(database_url);

    let addr = env::var("LISTEN_ADDRESS")
        .map(|x| x.parse().expect("LISTEN_ADDRESS provided is invalid"))
        .unwrap_or(([127, 0, 0, 1], 3000).into());

    let root_node = Arc::new(RootNode::new(Query, Mutation));

    let new_service = move || {
        let root_node = root_node.clone();
        let pool = pool.clone();
        service_fn(move |req| -> Box<Future<Item = _, Error = _> + Send> {
            let root_node = root_node.clone();
            let auth = req
                .headers()
                .get(AUTHORIZATION)
                .map(|header| header.to_str().unwrap().to_string().parse::<i32>().unwrap());
            let connection = Arc::new(Context {
                user_id: auth,
                database: pool.clone(),
            });
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/playground") => Box::new(juniper_hyper::playground("/")),
                (&Method::GET, "/") => Box::new(juniper_hyper::graphql(root_node, connection, req)),
                (&Method::POST, "/") => {
                    Box::new(juniper_hyper::graphql(root_node, connection, req))
                }
                _ => {
                    let mut response = Response::new(Body::empty());
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    Box::new(future::ok(response))
                }
            }
        })
    };
    let server = Server::bind(&addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));
    println!("Listening on http://{}", addr);

    rt::run(server);
}
