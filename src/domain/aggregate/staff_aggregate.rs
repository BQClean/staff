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

#[derive(Deserialize, Serialize, Clone)]
pub struct RootStaff {
   pub id: uuid::Uuid,
   pub first_name: String,
   pub last_name: String,
   pub vehicle_reg: String,
   pub driver_license: String,
   pub in_contract: bool,
   pub active: bool,
   pub address: Vec<Address>,
   pub contacts: Vec<Contact>,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Contact {
    pub id: String,
    pub contact_type_id: String,
    pub contact_value: String,
    pub staff_id:String,
    pub primary:bool
}
#[derive(Deserialize, Serialize, Clone)]
pub struct Address {
    pub id: String,
    pub street: String,
    pub state: String,
    pub post_code: String,
    pub country: String,
    pub staff_id: String,
    pub primary:bool
}
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