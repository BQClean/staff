pub trait IConfig {
    fn get_dbconnection(&self)->String;
    fn get_store_connection(&self)->String;
    fn get_service_address(&self)->String;
}