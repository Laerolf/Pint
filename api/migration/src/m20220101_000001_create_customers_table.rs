use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Customers {
    Table,
    Id,
    FirstName,
    LastName,
    Nickname,
    EmailAddress,
    DateOfBirth,
    Source,
    SourceId,
    CreatedAt,
    LastUpdatedAt,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Customers::Table)
                    .if_not_exists()
                    .col(pk_auto(Customers::Id))
                    .col(string_null(Customers::FirstName))
                    .col(string_null(Customers::LastName))
                    .col(string_null(Customers::Nickname))
                    .col(string_null(Customers::EmailAddress))
                    .col(date_null(Customers::DateOfBirth))
                    .col(string_null(Customers::Source))
                    .col(string_null(Customers::SourceId))
                    .col(date(Customers::CreatedAt))
                    .col(date_null(Customers::LastUpdatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_customers_source_source_id")
                    .table(Customers::Table)
                    .col(Customers::Source)
                    .col(Customers::SourceId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Customers::Table).to_owned())
            .await
    }
}
