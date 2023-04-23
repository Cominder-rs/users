pub mod utils;
mod api;
mod middlewares;

use tonic::{transport::Server, Request, Status};

use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

use middlewares::AppDataMiddlewareLayer;
use api::v1::users::UsersApi;

use users_proto::{users_server::UsersServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let addr = "192.168.0.103:50501".parse()?;

    let users_api = UsersApi {};

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let service = UsersServer::new(users_api);

    let layer = tower::ServiceBuilder::new()
        .layer(AppDataMiddlewareLayer::default())
        .into_inner();

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
