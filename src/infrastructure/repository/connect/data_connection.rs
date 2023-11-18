use async_trait::async_trait;
use sea_orm::*;
use crate::infrastructure::traits::trait_connection::TConnection;

pub struct Connection();

#[async_trait]
impl<'a> TConnection<'a> for Connection {
    async fn connect(&'a self, conn_url: &str) -> Result<DatabaseConnection, DbErr> {
        let mut opts = ConnectOptions::new(conn_url);
        opts.sqlx_logging(false);

        return Database::connect(opts).await;
    }
}