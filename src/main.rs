mod api;
mod middlewares;
pub mod utils;
pub mod db;

use std::sync::Arc;

use civilization::init_service;
use parking_lot::Mutex;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

use api::v1::users::UsersApi;
use middlewares::AppDataMiddlewareLayer;

use users_proto::users_server::UsersServer;

use crate::db::config::init_db;

const IPV4BIN: &str = "services/users/assets/IP2LOCATION-LITE-DB1.BIN";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = init_service();
    
    let pod_ip = std::env::var("POD_IP").unwrap_or("0.0.0.0".to_string());

    let addr = pod_ip + ":80";
    let addr = addr.parse().unwrap();

    let users_api = UsersApi {};

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let ip_db = ip2location::DB::from_file(IPV4BIN).expect("Opening IP2Location file");
    
    let app_data_middleware = AppDataMiddlewareLayer {
        env,
        ip_db: Arc::new(Mutex::new(ip_db)),
        db: init_db(env).await,
    };
    
    let service = UsersServer::new(users_api);
    let layer = tower::ServiceBuilder::new()
        .layer(app_data_middleware)
        .into_inner();

    tracing::event!(tracing::Level::INFO, "Users app is ready!");

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .layer(layer)
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}
