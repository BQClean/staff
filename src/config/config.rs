
use infrastructure::config::trait_config as tcfg;
use crate::infrastructure;

#[derive(Debug,Default)]
pub struct Config {
    database_host: String,
    database_port: String,
    database_username: String,
    database_password: String,
    database_name: String,
}

impl Config {
    fn new()-> Config{
        return Config{
            database_host: "localhost".to_string(),
            database_port: "5432".to_string(),
            database_username: "postgres".to_string(),
            database_password: "root".to_string(),
            database_name: "cleanx_staff".to_string(),
        }
    }
}
impl tcfg::TConfig for Config{
    fn get_dbconnection() -> String {
        todo!()
    }
}