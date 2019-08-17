use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use futures::future;
use hyper::header::AUTHORIZATION;
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::Method;
use hyper::{Body, Error, Request, Response, Server, StatusCode};
use jsonwebtoken::{decode, Algorithm, Validation};
use juniper::RootNode;
use log::*;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

use permissible_server::database::create_pool;
use permissible_server::graphql::context::Context;
use permissible_server::graphql::mutation::Mutation;
use permissible_server::graphql::query::Query;
use permissible_server::jwt::Claims;

fn graphql_response(
    root_node: Arc<RootNode<'static, Query, Mutation>>,
    db_pool: Pool<ConnectionManager<SqliteConnection>>,
    req: Request<Body>,
) -> Box<Future<Item = Response<Body>, Error = Error> + Send> {
    let request_uuid = Uuid::new_v4();
    let mut user_id = None;

    info!("{} - request start", request_uuid);

    let auth = req.headers().get(AUTHORIZATION);
    if let Some(header) = auth {
        if let Ok(header_str) = header.to_str() {
            if let Ok(data) = decode::<Claims>(
                header_str,
                &env::var("JWT_SECRET").unwrap().into_bytes(),
                &Validation::new(Algorithm::HS256),
            ) {
                info!("{} - request is user {}", request_uuid, data.claims.user_id);
                user_id = Some(data.claims.user_id);
            }
        }
    }

    let connection = Arc::new(Context {
        user_id: user_id,
        database: db_pool,
        request_uuid,
    });
    Box::new(
        juniper_hyper::graphql(root_node, connection, req).map(move |response| {
            info!("{} - request completed {}", request_uuid, response.status());
            response
        }),
    )
}

fn main() {
    dotenv().ok();
    env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    pretty_env_logger::init_timed();

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
            let pool = pool.clone();
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/playground") => Box::new(juniper_hyper::playground("/")),
                (&Method::GET, "/") => graphql_response(root_node, pool, req),
                (&Method::POST, "/") => graphql_response(root_node, pool, req),
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
