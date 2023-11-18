use sea_orm::{DatabaseConnection, DbErr};

pub trait TConnection{
    async fn connect(&mut self,conn_url :&String)->Result<DatabaseConnection,DbErr>;
}