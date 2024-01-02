use std::sync::Arc;
use rdkafka::consumer::StreamConsumer;
use rdkafka::Message;
use tokio_stream::StreamExt;

use crate::traits::trait_processor::{IMessageProcessor};

pub struct Handler();
impl Handler{
    pub fn new()->Self{
        return Handler()
    }
}

impl IMessageProcessor for Handler{
    async fn handler(consumer: StreamConsumer) {
       while let Some(Ok(msg))=consumer.stream().next().await{
            if let Some(Ok(data))=msg.payload_view::<str>(){
                println!("{:?}",data)
            }
       }
    }
}