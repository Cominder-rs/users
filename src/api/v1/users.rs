use crate::{middlewares::{AppData}, utils};
use sea_orm::{prelude::*, Set};
use tonic::{Request, Response, Status};
use tracing::*;
use users_proto::{users_server::Users, Country, CountryCode, PhoneNumber, SentCode};
use users_entities::login_sessions::{ActiveModel as NewLoginSession};
use rand::{self, Rng};

pub struct UsersApi;

#[tonic::async_trait]
impl Users for UsersApi {
    async fn get_country(&self, request: Request<()>) -> Result<Response<Country>, Status> {
        debug!("Ok it;s some req");
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
        let AppData { ref db, .. } = request.extensions().get::<AppData>().unwrap();
        let phone_number = &request.get_ref().phone_number;

        let code = {
            let mut rng = rand::thread_rng();
            let mut code = String::with_capacity(6);

            for _ in 0..6 {
                let num = rng.gen_range(0..=9);
                code += &num.to_string()
            }
            code
            
        };

        
        
        let now = time::OffsetDateTime::now_utc();
        let expired_at = now + time::Duration::minutes(4);
        let new_login_session = NewLoginSession {
            id: Set(uuid::Uuid::new_v4()),
            phone_number: Set(phone_number.to_owned()),
            code: Set(code),
            expire_at: Set(expired_at.unix_timestamp()),
            sent_at: Set(now.unix_timestamp()),
            attempts: Set(0),
        };

        let res = new_login_session.insert(db).await;

        if let Err(err) = res {
            error!("Error insertion new login session {:?}", err);
            Err(Status::internal("Internal server error"))
        } else {
            Ok(Response::new(()))
        }

        
        
    }
    async fn send_phone_number_dev(
        &self,
        request: Request<PhoneNumber>,
    ) -> Result<Response<SentCode>, Status> {
        let AppData { ref db, .. } = request.extensions().get::<AppData>().unwrap();
        let phone_number = &request.get_ref().phone_number;

        let code = {
            let mut rng = rand::thread_rng();
            let mut code = String::with_capacity(6);

            for _ in 0..6 {
                let num = rng.gen_range(0..=9);
                code += &num.to_string()
            }
            code
            
        };

        
        
        let now = time::OffsetDateTime::now_utc();
        let expired_at = now + time::Duration::minutes(4);
        let new_login_session = NewLoginSession {
            id: Set(uuid::Uuid::new_v4()),
            phone_number: Set(phone_number.to_owned()),
            code: Set(code.clone()),
            expire_at: Set(expired_at.unix_timestamp()),
            sent_at: Set(now.unix_timestamp()),
            attempts: Set(0),
        };

        let res = new_login_session.insert(db).await;

        if let Err(err) = res {
            error!("Error insertion new login session {:?}", err);
            Err(Status::internal("Internal server error"))
        } else {
            Ok(Response::new(SentCode {
                code
            }))
        }
    }
}
