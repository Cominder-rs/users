use axum::{Extension, Router};
use std::net::SocketAddr;
use std::str::FromStr;

use tower_http::trace::TraceLayer;

use sea_orm::DatabaseConnection;

use api::v1::get_api;
use civilization::{init_service::init_service, common_structs::Env};

mod api;
mod db;

#[derive(Clone)]
pub struct AppState {
    db_conn: DatabaseConnection,
    env: Env,
}

#[tokio::main]
async fn main() {

    let (env, ..) = init_service();

    let db_conn = db::init_conn(&env).await;

    let app_state = AppState { db_conn, env };

    let app = Router::new()
        .nest("/api/v1", get_api())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));


    let addr = SocketAddr::from_str("127.0.0.1:8080").expect("Invalid URL passed");

    tracing::info!("Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
