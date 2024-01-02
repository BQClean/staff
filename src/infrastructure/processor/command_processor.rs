use std::sync::Arc;
use rdkafka::consumer::StreamConsumer;

use crate::traits::trait_processor::{IMessageProcessor};

pub struct Handler();
impl Handler{
    pub fn new()->Self{
        return Handler()
    }
}

impl IMessageProcessor for Handler{
    async fn handler(consumer: StreamConsumer) {
        todo!()
    }
}