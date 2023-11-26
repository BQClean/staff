pub trait TConfig {
    fn get_dbconnection(&self)->String;
    fn get_store_connection(&self)->String;
}