use sea_orm::{DatabaseConnection, DbErr};


pub trait TConnection<'a>{
    async fn connect(&'a self, conn_url :&str) ->Result<DatabaseConnection, DbErr>;
}