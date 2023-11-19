use sea_orm_migration::prelude::*;
use crate::migrator::m20220101_000001_staff_table::Staff;


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
         manager
            .create_table(
                Table::create()
                    .table(Address::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Address::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Address::Street).string().not_null())
                    .col(ColumnDef::new(Address::Suburb).string().not_null())
                    .col(ColumnDef::new(Address::State).string().not_null())
                    .col(ColumnDef::new(Address::PostCode).string().not_null())
                    .col(ColumnDef::new(Address::Country).string().not_null())
                    .col(ColumnDef::new(Address::StaffID).uuid().not_null())
                    .foreign_key(ForeignKey::create().name("fk-staff-address").
                        from(Address::Table,Address::StaffID).
                        to(Staff::Table,Staff::Id))
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Address::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Address {
    Table,
    Id,
    StaffID,
    Street,
    Suburb,
    State,
    PostCode,
    Country
}
