use std::sync::Arc;
use config::Config;
use rdkafka::ClientConfig;
use rdkafka::consumer::StreamConsumer;
use rdkafka::producer::FutureProducer;
use crate::traits::trait_config::IConfig;

#[derive(Clone)]
pub struct KafkaMessage<'a>  {
    cfg: &'a dyn IConfig
}


impl <'a>KafkaMessage<'a>  {
    pub fn new(cfg:&'a impl IConfig) -> Box<KafkaMessage> {
        let message = KafkaMessage {
            cfg
        };

        return Box::new(message);
    }

    pub(crate) fn create_consumer(&self) -> StreamConsumer {
        let consumer:StreamConsumer = ClientConfig::new()
            .set("group.id",self.cfg.get_kafka_staff_consumer_group())
            .set("bootstrap.servers",self.cfg.get_kafka_bootstrap_server())
            .set("sasl.mechanism",self.cfg.get_kafka_ssl_mechanism())
            .set("security.protocol",self.cfg.get_kafka_security_protocol())
            .set("sasl.username",self.cfg.get_kafka_sasl_username())
            .set("sasl.password",self.cfg.get_kafka_sasl_password())
            .create()
            .expect("consumer creation error");

        return consumer
    }

    pub(crate) fn create_producer(&self) -> FutureProducer {
        let producer:FutureProducer =  ClientConfig::new()
            .set("bootstrap.servers",self.cfg.get_kafka_bootstrap_server())
            .set("sasl.mechanism",self.cfg.get_kafka_ssl_mechanism())
            .set("security.protocol",self.cfg.get_kafka_security_protocol())
            .set("sasl.username",self.cfg.get_kafka_sasl_username())
            .set("sasl.password",self.cfg.get_kafka_sasl_password())
            .create()
            .expect("producer creation error");

        return producer
    }
}
