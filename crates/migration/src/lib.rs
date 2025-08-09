pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_mapping_table;
mod m20250806_081507_create_visitor_table;
mod m20250809_130823_create_trimmed;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_mapping_table::Migration),
            Box::new(m20250806_081507_create_visitor_table::Migration),
            Box::new(m20250809_130823_create_trimmed::Migration),
        ]
    }
}
