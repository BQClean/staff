use sea_orm::{DatabaseConnection, DbErr};
use crate::infrastructure::traits::trait_connection::TConnection;

pub struct Connection{
    connection_url:*String
}
impl  TConnection for Connection{
    async fn connect(&mut self,conn_url: &String) -> Result<DatabaseConnection, DbErr> {
        self.connection_url = conn_url;

        return Ok(None)
    }
}