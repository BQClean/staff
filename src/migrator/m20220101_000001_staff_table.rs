use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Staff::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Staff::Id)
                            .uuid()
                            .not_null()
                            .generated("uuid-ossp",true)
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Staff::FirstName).string().not_null())
                    .col(ColumnDef::new(Staff::LastName).string().not_null())
                    .col(ColumnDef::new(Staff::Address).string().not_null())
                    .col(ColumnDef::new(Staff::Mobile).string().not_null())
                    .col(ColumnDef::new(Staff::Land).string().null())
                    .col(ColumnDef::new(Staff::VehicleReg).string().null())
                    .col(ColumnDef::new(Staff::INContract).boolean().not_null())
                    .col(ColumnDef::new(Staff::CreatedAt).timestamp_with_time_zone().default("CURRENT_TIMESTAMP").not_null())
                    .col(ColumnDef::new(Staff::UpdatedAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Staff::Table).
            to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Staff {
    Table,
    Id,
    FirstName,
    LastName,
    Address,
    Mobile,
    Land,
    DriverLC,
    VehicleReg,
    INContract,
    CreatedAt,
    UpdatedAt
}
