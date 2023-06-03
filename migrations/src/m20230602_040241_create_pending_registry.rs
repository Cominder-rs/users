use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PendingRegistries::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PendingRegistries::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PendingRegistries::RandomKey).string().string_len(32).not_null())
                    .col(ColumnDef::new(PendingRegistries::PhoneNumber).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PendingRegistries::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum PendingRegistries {
    Table,
    Id,
    RandomKey,
    PhoneNumber,
}
