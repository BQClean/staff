use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use crate::domain::commands::{CommandsStaff, Staff};
use crate::domain::entities::{EntityAddress, EntityContract};
use crate::domain::events::{CommonEvent,
                            StaffError,
                            StaffEvent,
                            EventStaff,
                            EventAddress,
                            EventContact};

#[async_trait]
impl Aggregate for RootStaff {
    type Command = CommandsStaff;
    type Event = StaffEvent;
    type Error = StaffError;
    type Services = ();

    fn aggregate_type() -> String {
        return String::from("staff");
    }

    async fn handle(&self,command: Self::Command,service: &Self::Services) -> Result<Vec<Self::Event>, Self::Error> {
       match command {
           CommandsStaff::CreateStaff {
               data,
               id,
               recv_timestamp
           }=>{
               let staff_val=self.compose_staff(&data);
               Ok(vec![StaffEvent::StaffCreated(CommonEvent{
                   corelation_id: id,
                   data: staff_val,
                   recv_timestamp,
               })])
           },
           CommandsStaff::UpdateStaff {
               id,
               data,
               recv_timestamp
           }=>{
               let staff_val=self.compose_staff(&data);
               Ok(vec![StaffEvent::StaffUpdated(CommonEvent{
                   corelation_id: id,
                   data: staff_val,
                   recv_timestamp,
               })])
           },
           CommandsStaff::CreateAddress {
               id,
               data,
               recv_timestamp
           }=>{

           }
       }
    }

    fn apply(&mut self, event: Self::Event) {
        todo!()
    }
}

impl Default for RootStaff {
    fn default() -> Self {
        todo!()
    }
}