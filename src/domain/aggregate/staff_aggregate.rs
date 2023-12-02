use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use crate::domain::commands::{CommandsStaff, CmdStaff};
use crate::domain::entities::{EntityAddress, EntityContract};
use crate::domain::events::{CommonEvent,
                            StaffError,
                            StaffEvent,
                            EventStaff,
                            EventAddress,
                            EventContact};

#[async_trait]
impl Aggregate for AggStaff {
    type Command = CommandsStaff;
    type Event = StaffEvent;
    type Error = StaffError;
    type Services = ();

    fn aggregate_type() -> String {
        return String::from("staff");
    }

    async fn handle(&self, command: Self::Command, service: &Self::Services) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            CommandsStaff::CreateStaff {
                data,
                id,
                recv_timestamp
            } => {
                let staff_val = self.get_staff_event(Box::new(Some(data)));
                match staff_val {
                    Some(staff)=>{
                        Ok(vec![StaffEvent::StaffCreated(CommonEvent {
                            corelation_id: id,
                            data: staff,
                            recv_timestamp,
                        })])
                    },
                    None=>{
                       Err(StaffError("error in processing staff command".to_string()))
                    }
                }
            }
            CommandsStaff::UpdateStaff {
                id,
                data,
                recv_timestamp
            } => {
                let staff_val = self.get_staff_event(Box::new(Some(data)));
                match staff_val {
                    Some(staff)=>{
                        Ok(vec![StaffEvent::StaffUpdated(CommonEvent {
                            corelation_id: id,
                            data: staff,
                            recv_timestamp,
                        })])
                    },
                    None=>{
                        Err(StaffError("error in processing staff command".to_string()))
                    }
                }
            }
            CommandsStaff::CreateAddress {
                id,
                data,
                recv_timestamp
            } => {

                let staff_val = self.get_address_event(Box::new(Some(data)));
                match staff_val {
                    Some(staff)=>{
                        Ok(vec![StaffEvent::AddressCreated(CommonEvent {
                            corelation_id: id,
                            data: staff,
                            recv_timestamp,
                        })])
                    },
                    None=>{
                        Err(StaffError("error in processing staff command".to_string()))
                    }
                }
                
            }
        }
    }

    fn apply(&mut self, event: Self::Event) {
        todo!()
    }
}

impl Default for AggStaff {
    fn default() -> Self {
        todo!()
    }
}