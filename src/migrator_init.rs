use sea_orm_migration::{MigratorTrait};
use crate::infrastructure::repository::connect::data_connection::{DBConnection};
use crate::infrastructure::repository::connect::trait_connection::{TConnection};
use crate::migrator::{Migrator};

use crate::config::config::Config;
use traits::trait_config::IConfig;


mod config;
mod infrastructure;
mod migrator;

mod traits;
mod domain;
mod helpers;
mod services;

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>>{
    let cfg_default = Config::default();
    let connection_url = cfg_default.get_dbconnection();

    let conn = DBConnection::default();
    let db = match conn.connect(connection_url.as_str()).await {
        Ok(db)=>db,
        Err(err)=>panic!("{}",err)
    };

    match Migrator::up(&db, None).await {
        Err(err) => panic!("{}", err),
        Ok(_) => (),
    };

    Ok(())
}