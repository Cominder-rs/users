pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_users_table;
mod m20220726_113009_create_login_session;
mod m20230602_040241_create_pending_registry;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_users_table::Migration),
            Box::new(m20220726_113009_create_login_session::Migration),
            Box::new(m20230602_040241_create_pending_registry::Migration),
        ]
    }
}
