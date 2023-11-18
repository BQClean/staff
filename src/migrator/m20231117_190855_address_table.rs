use sea_orm_migration::prelude::*;
use crate::migrator::BinOper::Add;

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
                            .generated("uuid-ossp",true)
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Address::Street).string().not_null())
                    .col(ColumnDef::new(Address::Suburb).string().not_null())
                    .col(ColumnDef::new(Address::State).string().not_null())
                    .col(ColumnDef::new(Address::PostCode).string().not_null())
                    .col(ColumnDef::new(Address::Country).string().not_null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Address {
    Table,
    Id,
    Street,
    Suburb,
    State,
    PostCode,
    Country
}
