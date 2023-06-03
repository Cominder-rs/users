mod api;
pub mod db;
mod middlewares;
pub mod utils;

use std::{sync::Arc, time::Duration};

use civilization::init_service;
use http::{HeaderName, HeaderValue};
use parking_lot::Mutex;
use tonic::{transport::Server, Request, metadata::MetadataValue, Status, Response};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};

use api::v1::users::AuthApi;
use middlewares::AppDataMiddlewareLayer;

use users_proto::{auth_server::AuthServer, permissions_server::{Permissions, PermissionsServer}};

use crate::db::config::init_db;

const IPV4BIN: &str = "services/users/assets/IP2LOCATION-LITE-DB1.BIN";

const DEFAULT_MAX_AGE: Duration = Duration::from_secs(24 * 60 * 60);
const DEFAULT_EXPOSED_HEADERS: [&str; 3] =
    ["grpc-status", "grpc-message", "grpc-status-details-bin"];
const DEFAULT_ALLOW_HEADERS: [&str; 4] =
    ["x-grpc-web", "content-type", "x-user-agent", "grpc-timeout"];

struct PermissionsApi;

#[tonic::async_trait]
impl Permissions for PermissionsApi {
    async fn test(&self, req: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = init_service();

    let pod_ip = std::env::var("POD_IP").unwrap_or("0.0.0.0".to_string());

    let addr = pod_ip + ":80";

    let addr = addr.parse().unwrap();
    let users_api = AuthApi {};

    let cors = CorsLayer::new()
        .allow_origin(
            "https://www.constellation-project.ru"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_credentials(true)
        .max_age(DEFAULT_MAX_AGE)
        .expose_headers(
            DEFAULT_EXPOSED_HEADERS
                .iter()
                .cloned()
                .map(HeaderName::from_static)
                .collect::<Vec<HeaderName>>(),
        )
        .allow_headers(
            DEFAULT_ALLOW_HEADERS
                .iter()
                .cloned()
                .map(HeaderName::from_static)
                .collect::<Vec<HeaderName>>(),
        );

    let ip_db = ip2location::DB::from_file(IPV4BIN).expect("Opening IP2Location file");

    let app_data_middleware = AppDataMiddlewareLayer {
        env,
        ip_db: Arc::new(Mutex::new(ip_db)),
        db: init_db(env).await,
    };

    let service = AuthServer::new(users_api);
    let layer = tower::ServiceBuilder::new()
        .layer(app_data_middleware)
        .into_inner();
    let dummy_service = PermissionsServer::new(PermissionsApi {});

    tracing::event!(tracing::Level::INFO, "Users app is ready!");

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .layer(layer)
        .add_service(service)
        .add_service(dummy_service)
        .serve(addr)
        .await?;

    Ok(())
}

fn chech_auth(req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("Authorization") {
        Some(t) => {
            let token = t.to_str().map_err(|_| Status::unauthenticated(""))?;
            
            Ok(req)
        }
        None => Err(Status::unauthenticated("")),
    }
}