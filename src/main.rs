mod api;
pub mod db;
mod middlewares;
pub mod utils;

use std::sync::Arc;

use civilization::init_service;
use parking_lot::Mutex;
use tonic::{transport::Server, codegen::CompressionEncoding};
use tonic_web::GrpcWebLayer;


use api::v1::auth::AuthApi;
use middlewares::AppDataMiddlewareLayer;

use users_proto::{auth_server::AuthServer, users_v1_server::UsersV1Server};

use crate::{db::config::init_db, api::v1::users::UsersApiV1};

const IPV4BIN: &str = "services/users/assets/IP2LOCATION-LITE-DB1.BIN";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (env, addr, cors_layer) = init_service();

    let users_api = AuthApi {};

    let ip_db = ip2location::DB::from_file(IPV4BIN).expect("Opening IP2Location file");

    let app_data_middleware = AppDataMiddlewareLayer {
        env,
        ip_db: Arc::new(Mutex::new(ip_db)),
        db: init_db(env).await,
    };

    let auth_v1_service = AuthServer::new(users_api).send_compressed(CompressionEncoding::Gzip);
    let users_v1_service = UsersV1Server::new(UsersApiV1 {});
    let layer = tower::ServiceBuilder::new()
        .layer(app_data_middleware)
        .into_inner();

    tracing::event!(tracing::Level::INFO, "Users app is ready!");

    Server::builder()
        .accept_http1(true)
        .layer(cors_layer)
        .layer(GrpcWebLayer::new())
        .layer(layer)
        .add_service(auth_v1_service)
        .add_service(users_v1_service)
        .serve(addr)
        .await?;

    Ok(())
}

// fn chech_auth(req: Request<()>) -> Result<Request<()>, Status> {
//     match req.metadata().get("Authorization") {
//         Some(t) => {
//             let _token = t.to_str().map_err(|_| Status::unauthenticated(""))?;
            
//             Ok(req)
//         }
//         None => Err(Status::unauthenticated("")),
//     }
// }
