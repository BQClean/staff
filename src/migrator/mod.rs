use sea_orm::prelude::async_trait;
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_staff_table;
mod m20231117_190855_address_table;
mod m20231117_191129_contact_table;
mod m20231117_191218_contact_type_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_staff_table::Migration),
            Box::new(m20231117_190855_address_table::Migration),
            Box::new(m20231117_191129_contact_table::Migration),
            Box::new(m20231117_191218_contact_type_table::Migration),
        ]
    }
}
