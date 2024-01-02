use std::sync::Arc;
use async_trait::async_trait;
use rdkafka::consumer::StreamConsumer;
use rdkafka::Message;
use tokio_stream::StreamExt;

use crate::traits::trait_processor::{IMessageProcessor};

pub struct ProcessHandler{}
impl ProcessHandler{
    pub fn new()->Self{
        ProcessHandler{}
    }
}

#[async_trait]
impl IMessageProcessor for ProcessHandler{
    async fn handle(&self,consumer: Arc<StreamConsumer>) {
       while let Some(Ok(msg))=consumer.stream().next().await{
            if let Some(Ok(data))=msg.payload_view::<str>(){
                println!("{:?}",data)
            }
       }
    }
}