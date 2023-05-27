use civilization::common_structs::Env;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::log::LevelFilter;

pub async fn init_db(env: Env) -> DatabaseConnection {
    let connect_uri = match env {
        Env::Prod => std::env::var("DB_URI").expect("Set DB_URI var"),
        Env::Dev => {
            let user = "vaider";
            let password = "44668631";
            let host = "192.168.0.103";
            let database = "users";

            format!("postgres://{user}:{password}@{host}:5432/{database}")
        }
        Env::Test => {
            let user = "vaider";
            let password = "44668631";
            let host = "192.168.0.103";
            let database = "users_test";

            format!("postgres://{user}:{password}@{host}:5432/{database}")
        }
    };

    let mut opts = ConnectOptions::new(connect_uri);

    match env {
        Env::Prod => { opts.sqlx_logging(true).sqlx_logging_level(LevelFilter::Info); }
        Env::Dev => { opts.sqlx_logging(true).sqlx_logging_level(LevelFilter::Debug); }
        Env::Test => { opts.sqlx_logging(true).sqlx_logging_level(LevelFilter::Debug); }
    }
    

    Database::connect(opts)
        .await
        .expect("Unable to connect to database. Check connection details")
}
