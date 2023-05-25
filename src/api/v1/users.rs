use crate::{middlewares::AppData, utils};
use tonic::{Code, Request, Response, Status};
use tracing::*;
use users_proto::{users_server::Users, Country, CountryCode, PhoneNumber};

pub struct UsersApi;

#[tonic::async_trait]
impl Users for UsersApi {
    async fn get_country(&self, request: Request<()>) -> Result<Response<Country>, Status> {
        let AppData { ref ip_db, .. } = request.extensions().get::<AppData>().unwrap();

        let ip_addr = request.metadata().get("x-forwarded-for");
        if let Some(ip_addr) = ip_addr {
            let ip_addr = ip_addr.to_str().unwrap().parse().unwrap();

            let country = utils::find_country(ip_addr, ip_db.to_owned()).unwrap();

            let response = Country {
                code: country.into(),
            };

            Ok(Response::new(response))
        } else {
            warn!("No `x-forwarded-for` header");
            Ok(Response::new(Country {
                code: CountryCode::Unknown as i32,
            }))
        }
    }

    async fn send_phone_number(
        &self,
        request: Request<PhoneNumber>,
    ) -> Result<Response<()>, Status> {
        let phone_number = request.into_inner().phone_number;
        println!("{}", phone_number);
        Err(Status::new(Code::InvalidArgument, "Invalid phone number"))
    }
}
