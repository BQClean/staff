pub trait IConfig {
    fn get_dbconnection(&self)->String;
    fn get_store_connection(&self)->String;
    fn get_service_address(&self)->String;
    fn get_kafka_bootstrap_server(&self)->&str;
    fn get_kafka_ssl_mechanism(&self)->&str;
    fn get_kafka_security_protocol(&self)->&str;
    fn get_kafka_sasl_username(&self)->&str;
    fn get_kafka_sasl_password(&self)->&str;
    fn get_kafka_group_id(&self)->&str;
}