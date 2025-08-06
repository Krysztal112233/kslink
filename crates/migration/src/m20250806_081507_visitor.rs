use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_create_table::UrlMappingTable;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(VisitRecordTable::VisitRecord)
                    .if_not_exists()
                    .col(uuid_uniq(VisitRecordTable::Id).primary_key())
                    .col(string(VisitRecordTable::UserAgent))
                    .col(string(VisitRecordTable::RefHash))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_visitor_ref_hash")
                    .from(VisitRecordTable::VisitRecord, VisitRecordTable::RefHash)
                    .to(UrlMappingTable::UrlMapping, UrlMappingTable::Hash)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().if_exists().to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum VisitRecordTable {
    VisitRecord,

    Id,
    UserAgent,
    RefHash,
}
