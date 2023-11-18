use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CantactType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CantactType::Id)
                            .integer()
                            .not_null()
                            .generated("uuid-ossp",true)
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CantactType::Name).string().not_null())
                    .col(ColumnDef::new(CantactType::Description).string().null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CantactType::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum CantactType {
    Table,
    Id,
    Name,
    Description,
}
