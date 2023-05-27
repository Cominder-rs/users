use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220726_113009_create_login_session"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LoginSessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LoginSessions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LoginSessions::PhoneNumber)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(LoginSessions::Code).string().not_null())
                    .col(
                        ColumnDef::new(LoginSessions::ExpireAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LoginSessions::SentAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LoginSessions::Attempts)
                            .small_integer()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LoginSessions::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum LoginSessions {
    Table,
    Id,
    PhoneNumber,
    Code,
    ExpireAt,
    SentAt,
    Attempts,
}
