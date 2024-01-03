use std::sync::Arc;
use async_trait::async_trait;
use rdkafka::consumer::StreamConsumer;
use tokio::sync::Mutex;

#[async_trait]
pub trait IMessageProcessor {
   async fn  handle(&self, consumer: Arc<Mutex<StreamConsumer>>);
}