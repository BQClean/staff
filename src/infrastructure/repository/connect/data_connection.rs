use async_trait::async_trait;
use sea_orm::*;
use crate::traits::trait_connection::TConnection;


#[derive(Default)]
pub struct DBConnection();

#[async_trait]
impl<'a> TConnection<'a> for DBConnection {
    async fn connect(&'a self, conn_url: &str) -> Result<DatabaseConnection, DbErr> {
        let mut opts = ConnectOptions::new(conn_url);
        opts.sqlx_logging(false);
        opts.set_schema_search_path("public");
        return Database::connect(opts).await;
    }


}