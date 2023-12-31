// use std::sync::Arc;
// use rdkafka::consumer::StreamConsumer;
// use rdkafka::producer::FutureProducer;
//
// // interface for the kafka producer and consumer
// pub trait IKafka {
//     fn get_consumer(&self)->Arc<StreamConsumer>;
//     fn get_producer(&self)->Arc<FutureProducer>;
// }