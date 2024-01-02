#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]


use rdkafka::consumer::Consumer;
use sea_orm_migration::MigratorTrait;
use tokio::signal::unix::{signal, SignalKind};
use tokio::spawn;
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Receiver, Sender};
use tonic::transport::Server;
use crate::config::config::Config;
use traits::trait_config::IConfig;
use crate::infrastructure::repository::connect::data_connection::DBConnection;
use traits::trait_connection::TConnection;
use crate::application::api::{StaffServiceApi};
use crate::pbstaff::staff_server_service_server::StaffServerServiceServer;
use crate::adapters::messages::kafka_messages::{KafkaMessage};
use crate::traits::trait_kafka::IKafka;
use crate::infrastructure::processor::command_processor::{ProcessHandler};
use crate::traits::trait_processor::IMessageProcessor;

mod adapters;
mod application;
mod config;
mod domain;
mod infrastructure;
mod helpers;
mod services;
mod traits;
mod common;
mod queries;


pub mod pbstaff{
    include!("../staffserver/v1/staffserver.v1.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::default();
    let connection_url = cfg.get_dbconnection();

    let conn = DBConnection::default();
    let db = match conn.connect(connection_url.as_str()).await {
        Ok(db)=>db,
        Err(err)=>panic!("{}",err)
    };

    let service_address = cfg.get_service_address();
    let api_server = StaffServiceApi::new();

    let(signal_tx,signal_rx) =signal_channel();
    spawn(wait_for_sigterm(signal_tx));


    let consumer_task =tokio::spawn(async {
        let kafka=KafkaMessage::new(&cfg);
        let consumer=kafka.get_consumer();
        let handlerProcess = ProcessHandler::new();

        *consumer.subscribe(&[cfg.get_kafka_staff_topic().as_str()])?;

        handlerProcess.handle(consumer).await
    });

    Server::builder().add_service(
        StaffServerServiceServer::new(api_server))
        .serve_with_shutdown(service_address.parse()?,async {
       signal_rx.await.ok();
    }).await?;

    let _ = consumer_task.await;

   Ok(())
}
fn signal_channel()->(Sender<()>,Receiver<()>){
 oneshot::channel()
}
async fn wait_for_sigterm(tx:Sender<()>){
    let _ = signal(SignalKind::terminate())
        .expect("failed to install signal handler")
        .recv().await;

    println!("SIGTERM received : shutting down");
    let _ = tx.send(());
}
