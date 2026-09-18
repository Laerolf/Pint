use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveIden)]
enum Venues {
    Table,
    Id,
    Name,
    Country,
    LanguageCode,
    Timezone,
    Currency,
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
                    .table(Venues::Table)
                    .if_not_exists()
                    .col(pk_auto(Venues::Id))
                    .col(string_null(Venues::Name))
                    .col(string_null(Venues::Country))
                    .col(string_null(Venues::LanguageCode))
                    .col(string_null(Venues::Timezone))
                    .col(string_null(Venues::Currency))
                    .col(string_null(Venues::Source))
                    .col(string_null(Venues::SourceId))
                    .col(date_time(Venues::CreatedAt))
                    .col(date_time_null(Venues::LastUpdatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_venues_source_source_id")
                    .table(Venues::Table)
                    .col(Venues::Source)
                    .col(Venues::SourceId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Venues::Table).to_owned())
            .await
    }
}
