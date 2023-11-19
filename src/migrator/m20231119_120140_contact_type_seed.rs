use sea_orm_migration::prelude::*;
use uuid::Uuid;
use crate::migrator::m20231117_191129_contact_type_table::ContactType;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert_one = Query::insert()
            .into_table(ContactType::Table)
            .columns([ContactType::Id, ContactType::Name, ContactType::Description])
            .values_panic([Uuid::new_v4().into(), "EMAIL".into(), "staff email address type".into()])
            .to_owned();

        let insert_two = Query::insert()
            .into_table(ContactType::Table)
            .columns([ContactType::Id, ContactType::Name, ContactType::Description])
            .values_panic([Uuid::new_v4().into(), "MOBILE".into(), "staff mobile phone type".into()])
            .to_owned();

        let insert_three = Query::insert()
            .into_table(ContactType::Table)
            .columns([ContactType::Id, ContactType::Name, ContactType::Description])
            .values_panic([Uuid::new_v4().into(), "LAND".into(), "staff land phone type".into()])
            .to_owned();

        manager.exec_stmt(insert_one).await?;
        manager.exec_stmt(insert_two).await?;
        manager.exec_stmt(insert_three).await?;

        return Ok(());
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete_one = Query::delete().from_table(ContactType::Table).to_owned();
        manager.exec_stmt(delete_one).await?;

        return Ok(());
    }
}


