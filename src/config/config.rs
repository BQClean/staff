
use crate::infrastructure::traits::trait_config as cfg;

#[derive(Debug)]
pub struct Config {
    database_host: String,
    database_username: String,
    database_password: String,
    database_name: String,
    database_schema:String,
}

impl Default for Config{
    fn default() -> Self {
        return Config{
            database_host: "localhost".to_string(),
            database_username: "postgres".to_string(),
            database_password: "root".to_string(),
            database_name: "postgres".to_string(),
            database_schema: "cleanx_staff".to_string(),
        }
    }
}

impl cfg::TConfig for Config{
    fn get_dbconnection(&self) -> String {
        return  format!("postgres://{}:{}@{}/{}?currentSchema={}",
                        self.database_username,
                        self.database_password,
                        self.database_host,
                        self.database_name,self.database_schema)
    }
}