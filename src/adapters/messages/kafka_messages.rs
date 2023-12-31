// use std::sync::Arc;
// use rdkafka::ClientConfig;
// use rdkafka::consumer::StreamConsumer;
// use rdkafka::producer::FutureProducer;
// use crate::traits::trait_config::IConfig;
// use crate::traits::trait_kafka::IKafka;
//
// #[derive(Default, Clone)]
// pub struct KafkaMessage {
//     consumer: Arc<StreamConsumer>,
//     producer: Arc<FutureProducer>,
// }
//
// impl KafkaMessage {
//     pub fn new<T: IConfig>(config: &T) -> Self {
//         let message = KafkaMessage {
//             producer:Arc::new(Self::create_producer(config)),
//             consumer:Arc::new(Self::create_consumer(config)),
//         };
//
//         return message;
//     }
//
//     pub(crate) fn create_consumer<T: IConfig>(cfg: &T) -> StreamConsumer {
//         let consumer:StreamConsumer = ClientConfig::new()
//             .set("group.id",cfg.get_kafka_staff_consumer_group())
//             .set("bootstrap.servers",cfg.get_kafka_bootstrap_server())
//             .set("sasl.mechanism",cfg.get_kafka_ssl_mechanism())
//             .set("security.protocol",cfg.get_kafka_security_protocol())
//             .set("sasl.username",cfg.get_kafka_sasl_username())
//             .set("sasl.password",cfg.get_kafka_sasl_password())
//             .create()
//             .expect("consumer creation error");
//
//         return consumer
//     }
//
//     pub(crate) fn create_producer<T: IConfig>(cfg: &T) -> FutureProducer {
//         let producer:FutureProducer =  ClientConfig::new()
//             .set("bootstrap.servers",cfg.get_kafka_bootstrap_server())
//             .set("sasl.mechanism",cfg.get_kafka_ssl_mechanism())
//             .set("security.protocol",cfg.get_kafka_security_protocol())
//             .set("sasl.username",cfg.get_kafka_sasl_username())
//             .set("sasl.password",cfg.get_kafka_sasl_password())
//             .create()
//             .expect("producer creation error");
//
//         return producer
//     }
// }
//
// impl IKafka for KafkaMessage {
//     fn get_consumer(&self) -> Arc<StreamConsumer> {
//         self.consumer.clone()
//     }
//     fn get_producer(&self) -> Arc<FutureProducer> {
//         self.producer.clone()
//     }
// }