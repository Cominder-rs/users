use crate::{
    middlewares::AppData,
    utils,
};
use civilization::utils::{validate_regex, validate_vec_regex};
use ::users_proto::{auth_server::Auth, *};
use beijing::*;
use civilization::{common_structs::Env, utils::unix_now};
use http::status::InvalidStatusCode;
use rand::{self, Rng};
use sea_orm::{prelude::*, sea_query::OnConflict, Set};
use std::time::Duration;
use tonic::{Request, Response, Status};
use tracing::*;
use users_entities::{
    login_sessions::{self, ActiveModel as NewLoginSession, Entity as LoginSession},
    user::{self, Entity as User},
};
use users_errors::*;
use validator::{validate_does_not_contain, validate_length, validate_must_match, validate_phone};

pub struct AuthApi;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz\
                         0123456789)(*&^%$#@!~";

#[tonic::async_trait]
impl Auth for AuthApi {
    async fn create_user(&self, request: Request<NewUser>) -> Result<Response<Token>, Status> {
        let AppData { db, .. } = request.extensions().get::<AppData>().unwrap();
        let db = db.to_owned();
        let NewUser {
            username,
            firstname,
            lastname,
            city,
        } = request.into_inner();
        let regex = r"^(\p{L}(?<!\s)\s?){0,30}$";
        validate_regex(&username, regex)
            .map_err(|_| Status::invalid_argument(AuthError::InvalidUsername.to_string()))?;
        validate_regex(&firstname, regex)
            .map_err(|_| Status::invalid_argument(AuthError::InvalidFirstname.to_string()))?;
        validate_regex(&lastname, regex)
            .map_err(|_| Status::invalid_argument(AuthError::InvalidLastname.to_string()))?;
        validate_regex(&city, regex)
            .map_err(|_| Status::invalid_argument(AuthError::InvalidCity.to_string()))?;

        let new_user = user::ActiveModel {
            username: Set(username),
            firstname: Set(firstname),
            lastname: Set(lastname),
            city: Set(city),
            ..Default::default()
        };

        let new_user_id = user::Entity::insert(new_user)
            .on_conflict(
                OnConflict::column(user::Column::Username)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(&db)
            .await
            .map_err(|e| {
                if let DbErr::RecordNotInserted = e {
                    Status::already_exists(AuthError::UsernameBusy.to_string())
                } else {
                    Status::internal("")
                }
            })?
            .last_insert_id;

        let claims = Claims::new(new_user_id, None, Scope::all());
        let token = encode_token(&claims).map_err(|_| Status::internal(""))?;
        Ok(Response::new(Token {
            token
        }))
    }

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
    async fn send_confirmation_code(
        &self,
        request: Request<ConfirmationCode>,
    ) -> Result<Response<RegistryStatus>, Status> {
        let AppData { db, .. } = request.extensions().get::<AppData>().unwrap();
        let ConfirmationCode {
            code, random_key, ..
        } = request.get_ref();
        if code.len() != 6 {
            return Err(Status::invalid_argument(
                AuthError::InvalidConfirmationCode.to_string(),
            ));
        }

        let login_session = LoginSession::find()
            .filter(login_sessions::Column::RandomKey.eq(random_key))
            .one(db)
            .await
            .map_err(|_| Status::internal("Internal"))?
            .ok_or(Status::invalid_argument(
                AuthError::LoginSessionNotFound.to_string(),
            ))?;

        if login_session.code != *code {
            return Err(Status::invalid_argument(
                AuthError::InvalidConfirmationCode.to_string(),
            ));
        }

        let user = User::find()
            .filter(user::Column::PhoneNumber.eq(login_session.phone_number))
            .one(db)
            .await
            .map_err(|_| Status::internal("Internal"))?;

        if let Some(user) = user {
            let claims = Claims::new(user.id, None, Scope::all());
            let token = encode_token(&claims).map_err(|_| Status::internal(""))?;
            Ok(Response::new(RegistryStatus {
                is_done: true,
                token: Some(token),
                random_key: None,
            }))
        } else {
            let random_key = (0..32)
                .map(|_| {
                    let idx = {
                        let mut rng = rand::thread_rng();
                        rng.gen_range(0..CHARSET.len())
                    };

                    CHARSET[idx] as char
                })
                .collect::<String>();

            Ok(Response::new(RegistryStatus {
                is_done: false,
                token: None,
                random_key: Some(random_key),
            }))
        }
    }

    async fn send_phone_number(
        &self,
        request: Request<PhoneNumber>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("Unimplemented"))
        // let AppData { ref db, .. } = request.extensions().get::<AppData>().unwrap();

        // let phone_number = &request.get_ref().phone_number;
        // if !validate_phone(phone_number) {
        //     return Err(Status::invalid_argument(
        //         AuthError::InvalidPhoneNumber.to_string(),
        //     ));
        // };
        // let code = {
        //     let mut rng = rand::thread_rng();
        //     let mut code = String::with_capacity(6);

        //     for _ in 0..6 {
        //         let num = rng.gen_range(0..=9);
        //         code += &num.to_string()
        //     }
        //     code
        // };

        // let now = time::OffsetDateTime::now_utc();
        // let expired_at = now + time::Duration::minutes(4);
        // let new_login_session = NewLoginSession {
        //     id: Set(uuid::Uuid::new_v4()),
        //     phone_number: Set(phone_number.to_owned()),
        //     code: Set(code),
        //     expire_at: Set(expired_at.unix_timestamp()),
        //     sent_at: Set(now.unix_timestamp()),
        //     attempts: Set(0),
        // };

        // let res = new_login_session.insert(db).await;

        // if let Err(err) = res {
        //     error!("Error insertion new login session {:?}", err);
        //     Err(Status::internal("Internal server error"))
        // } else {
        //     Ok(Response::new(()))
        // }
    }
    async fn send_phone_number_dev(
        &self,
        request: Request<PhoneNumber>,
    ) -> Result<Response<CodeRandomKey>, Status> {
        let AppData {
            ref db, ref env, ..
        } = request.extensions().get::<AppData>().unwrap();
        if env == &Env::Prod {
            return Err(Status::aborted("Aborted"));
        }
        let phone_number = &request.get_ref().phone_number;
        if !validate_phone(phone_number) {
            return Err(Status::invalid_argument(
                AuthError::InvalidPhoneNumber.to_string(),
            ));
        };
        let code = {
            let mut rng = rand::thread_rng();
            let mut code = String::with_capacity(6);

            for _ in 0..6 {
                let num = rng.gen_range(0..=9);
                code += &num.to_string()
            }
            code
        };

        let random_key = (0..32)
            .map(|_| {
                let idx = {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(0..CHARSET.len())
                };

                CHARSET[idx] as char
            })
            .collect::<String>();

        let now = time::OffsetDateTime::now_utc();
        let expired_at = now + time::Duration::minutes(4);
        let new_login_session = NewLoginSession {
            id: Set(uuid::Uuid::new_v4()),
            phone_number: Set(phone_number.to_owned()),
            code: Set(code.clone()),
            expire_at: Set(expired_at.unix_timestamp()),
            sent_at: Set(now.unix_timestamp()),
            attempts: Set(0),
            random_key: Set(random_key.clone()),
        };

        let res = new_login_session.insert(db).await;

        if let Err(err) = res {
            error!("Error insertion new login session {:?}", err);
            Err(Status::internal("Internal server error"))
        } else {
            Ok(Response::new(CodeRandomKey { code, random_key }))
        }
    }
}
