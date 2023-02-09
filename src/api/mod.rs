// use tonic::{Request, Response, Status};
// use civilization::scopes;
// use crate::users::{HelloReply, HelloRequest};
//
// #[scopes]
// pub async fn say_hello(
//     request: Request<HelloRequest>, // Accept request of type HelloRequest
// ) -> Result<Response<HelloReply>, Status> { // Return an instance of type HelloReply
//
//     let reply = HelloReply {
//         message: format!("Hello {}!", request.into_inner().name), // We must use .into_inner() as the fields of gRPC requests and responses are private
//     };
//
//     Ok(Response::new(reply)) // Send back our formatted greeting
// }
