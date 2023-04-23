use users_proto::{
    users_server::Users,
    Country,
    PhoneNumber,
    CountryCode,
};
use tonic::{Request, Response, Status, Code};
use crate::{utils, middlewares::AppData};


pub struct UsersApi;

#[tonic::async_trait]
impl Users for UsersApi {
    async fn get_country(&self, request: Request<()>) -> Result<Response<Country>, Status> {
        let AppData{ ref ip_db, ..}= request.extensions().get::<AppData>().unwrap();
        let ip_addr = request.remote_addr();
        let country = utils::find_country(ip_addr, ip_db.clone()).unwrap();

        println!("{}", country as i32);
        let response = Country {
            code: country.into()
        };

        Ok(Response::new(response))
    }

    async fn send_phone_number(&self, request: Request<PhoneNumber>) -> Result<Response<()>, Status> {
        let phone_number = request.into_inner().phone_number;
        println!("{}", phone_number);
        Err(Status::new(Code::InvalidArgument, "Invalid phone number"))
    }
}
