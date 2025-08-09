use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_mapping_table::UrlMappingTable;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(UrlMappingTable::UrlMapping)
                    .add_column(
                        ColumnDef::new(AlterUrlMappingTable::Trimmed)
                            .not_null()
                            .json_binary()
                            .default(Expr::cust("'{}'::json")),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(UrlMappingTable::UrlMapping)
                    .drop_column(AlterUrlMappingTable::Trimmed)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum AlterUrlMappingTable {
    Trimmed,
}
