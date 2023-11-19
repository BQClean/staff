use sea_orm_migration::prelude::*;
use crate::migrator::m20231117_191218_contact_type_table::ContactType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Contact::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Contact::Id)
                            .integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Contact::ContactTypeID).uuid().not_null())
                    .col(ColumnDef::new(Contact::ContactValue).string().not_null())
                    .foreign_key(
                        ForeignKey::create().name("fk-contact-type").from(Contact::Table,
                             Contact::ContactTypeID)
                            .to(ContactType::Table,ContactType::Id)
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Contact::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Contact {
    Table,
    Id,
    ContactTypeID,
    ContactValue,
}
