use async_trait::async_trait;
use rdkafka::consumer::StreamConsumer;

#[async_trait]
pub trait IMessageProcessor {
   async fn  handler(consumer: StreamConsumer);
}