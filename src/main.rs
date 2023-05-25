mod api;
mod middlewares;
pub mod utils;

use tokio::signal::unix::{signal, SignalKind};
use tonic::{transport::Server, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};

use api::v1::users::UsersApi;
use middlewares::AppDataMiddlewareLayer;

use users_proto::users_server::UsersServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let pod_ip = std::env::var("POD_IP").unwrap_or("0.0.0.0".to_string());

    let addr = pod_ip + ":80";
    let addr = addr.parse().unwrap();

    let users_api = UsersApi {};

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    let service = UsersServer::new(users_api);

    let layer = tower::ServiceBuilder::new()
        .layer(AppDataMiddlewareLayer::default())
        .into_inner();

    tracing::event!(tracing::Level::INFO, "Users app is ready!");

    tokio::task::spawn(async {
        let mut sig = signal(SignalKind::terminate()).expect("Creating signal");
        sig.recv().await;
        tracing::event!(tracing::Level::INFO, "Cleaning socket");
        std::process::exit(0);
    });

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
