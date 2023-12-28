use std::sync::Arc;
use postgres_es::PostgresCqrs;
use sqlx::{Pool, Postgres};
use crate::config::trait_config::{IConfig};

#[derive(Debug)]
pub struct Config<'a> {
    database_host: String,
    database_username: String,
    database_password: String,
    database_name: String,
    database_schema: String,
    event_store_name: String,
    event_store_schema: String,
    service_host: String,
    service_port: String,

    kafka_bootstrap_server:&'a str,
    kafka_ssl_mechanism:&'a str,
    kafka_security_protocol: &'a str,
    kafka_sasl_username: &'a str,
    kafka_sasl_password: &'a str,
    kafka_group_id: & 'a str

}

impl <'a>Default for Config<'a> {
    fn default() -> Self {
        return Config {
            database_host: "localhost".to_string(),
            database_username: "postgres".to_string(),
            database_password: "root".to_string(),
            database_name: "cleanx".to_string(),
            database_schema: "cleanx_staff".to_string(),
            event_store_name: "event_store".to_string(),
            event_store_schema: "staff_store".to_string(),
            service_host: "127.0.0.1".to_string(),
            service_port: "50021".to_string(),

            kafka_bootstrap_server: "polished-scorpion-11453-us1-message.upstash.io:9092",
            kafka_ssl_mechanism: "SCRAM-SHA-256",
            kafka_security_protocol: "SASL_SSL",
            kafka_sasl_username: "cG9saXNoZWQtc2NvcnBpb24tMTE0NTMkIitTf9YL3Uuuf_vShTbeEJ6mgfSR-DU",
            kafka_sasl_password: "YzI3NGYxNzQtZDU5ZS00NjliLWJjYTctODRhYzcxMGFjYzAy",
            kafka_group_id: "racing_worker",
        };
    }
}

impl <'a>IConfig for Config<'a> {
    fn get_dbconnection(&self) -> String {
        return format!("postgres://{}:{}@{}/{}?currentSchema={}",
                       self.database_username,
                       self.database_password,
                       self.database_host,
                       self.database_name, self.database_schema);
    }
    fn get_store_connection(&self) -> String {
        return format!("postgres://{}:{}@{}/{}?currentSchema={}",
                       self.database_username,
                       self.database_password,
                       self.database_host,
                       self.event_store_name, self.event_store_schema);
    }
    fn get_service_address(&self) -> String {
        format!("{}:{}",self.service_host,self.service_port)
    }
}