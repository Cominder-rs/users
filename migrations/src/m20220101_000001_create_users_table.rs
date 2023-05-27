use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_users_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .big_integer()
                            .not_null()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Username).string().unique_key())
                    .col(ColumnDef::new(User::Password).string())
                    .col(
                        ColumnDef::new(User::PhoneNumber)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(User::Firstname).string().not_null())
                    .col(ColumnDef::new(User::Lastname).string().not_null())
                    .col(ColumnDef::new(User::Sex).string())
                    .col(ColumnDef::new(User::DateOfBirth).date())
                    .col(ColumnDef::new(User::City).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    PhoneNumber,
    Username,
    Password,
    Firstname,
    Lastname,
    Sex,
    DateOfBirth,
    City,
}
