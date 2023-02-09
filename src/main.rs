mod api;

use tonic::{transport::Server, Request, Response, Status};

use users::users_server::{Users, UsersServer};
use users::{HelloReply, HelloRequest, CodeRequest, Empty};

use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};
use civilization::scopes;


pub mod users {
    tonic::include_proto!("users"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyUsers {}

#[tonic::async_trait]
impl Users for MyUsers {

    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;

        let response = HelloReply {
            message: format!("Hello, {name}"),
        };

        Ok(Response::new(response))
    }

    // #[scopes]
    async fn send_code(
        &self,
        request: Request<CodeRequest>,
    ) -> Result<Response<Empty>, Status> {
        let CodeRequest { phone_number } = request.into_inner();

        println!("hui");
        Ok(Response::new(Empty {}))
    }
}

#[tokio::main]
async fn main () -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let addr = "127.0.0.1:50051".parse()?;
    let users = MyUsers::default();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(UsersServer::new(users))
        .serve(addr)
        .await?;

    Ok(())
}