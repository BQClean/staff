use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ContactType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ContactType::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ContactType::Name).string().not_null())
                    .col(ColumnDef::new(ContactType::Description).string().null())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ContactType::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
pub enum ContactType {
    Table,
    Id,
    Name,
    Description,
}
