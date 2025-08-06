use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UrlMappingTable::UrlMapping)
                    .if_not_exists()
                    .col(string_uniq(UrlMappingTable::Hash).primary_key())
                    .col(string_uniq(UrlMappingTable::Dest))
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .table(UrlMappingTable::UrlMapping)
                    .col(UrlMappingTable::Dest)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UrlMappingTable::UrlMapping).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum UrlMappingTable {
    UrlMapping,

    Hash,
    Dest,
}
