use async_trait::async_trait;
use sea_orm::{DatabaseConnection, DbErr};

#[async_trait]
pub trait TConnection<'a>{
    async fn connect(&'a self, conn_url :&str) ->Result<DatabaseConnection, DbErr>;
}