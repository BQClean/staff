use std::sync::Arc;
use postgres_es::PostgresCqrs;
use sqlx::{Pool, Postgres};
use crate::config::trait_config::{IConfig};
use crate::domain::aggregate::AggStaff;

#[derive(Debug)]
pub struct Config {
    database_host: String,
    database_username: String,
    database_password: String,
    database_name: String,
    database_schema:String,
    event_store_name:String,
    event_store_schema:String
}

impl Default for Config{
    fn default() -> Self {
        return Config{
            database_host: "localhost".to_string(),
            database_username: "postgres".to_string(),
            database_password: "root".to_string(),
            database_name: "cleanx".to_string(),
            database_schema: "cleanx_staff".to_string(),
            event_store_name: "event_store".to_string(),
            event_store_schema:"staff_store".to_string()
        }
    }
}

impl IConfig for Config{
     fn get_dbconnection(&self) -> String {
        return  format!("postgres://{}:{}@{}/{}?currentSchema={}",
                        self.database_username,
                        self.database_password,
                        self.database_host,
                        self.database_name,self.database_schema)
    }

    fn get_store_connection(&self) -> String {
        return  format!("postgres://{}:{}@{}/{}?currentSchema={}",
                        self.database_username,
                        self.database_password,
                        self.database_host,
                        self.event_store_name,self.event_store_schema)
    }

    fn get_cqrs_framework(pool:Pool<Postgres>)->Arc<PostgresCqrs<AggStaff>>{

    }

}