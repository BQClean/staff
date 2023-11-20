#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use sea_orm_migration::MigratorTrait;
use crate::config::config::Config;
use crate::infrastructure::traits::trait_config::TConfig;
use crate::infrastructure::repository::connect::data_connection::DBConnection;
use crate::infrastructure::traits::trait_connection::TConnection;

mod adapters;
mod application;
mod config;
mod domain;
mod entities;
mod infrastructure;
mod helpers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::default();
    let connection_url = cfg.get_dbconnection();

    let conn = DBConnection::default();
    let db = match conn.connect(connection_url.as_str()).await {
        Ok(db)=>db,
        Err(err)=>panic!("{}",err)
    };
   Ok(())
}