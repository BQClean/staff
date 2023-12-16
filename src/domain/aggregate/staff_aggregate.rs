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
use crate::services::svc_staff;
use crate::services::svc_staff::StaffService;
use crate::traits::IStaffService;

#[async_trait]
impl Aggregate for AggStaff {
    type Command = CommandsStaff;
    type Event = StaffEvent;
    type Error = StaffError;
    type Services = StaffService;

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
                let serve_valid = service.validate_staff(Box::new(Some(&data))).await;
                match serve_valid{
                    Ok(_)=>{
                        self.create_staff_match(&data, id, recv_timestamp).await
                    }
                    Err(e)=> return Err(StaffError("error in staff validation".to_string()))
                }
            }
            CommandsStaff::UpdateStaff {
                id,
                data,
                recv_timestamp
            } => {
                let serve_valid = service.validate_staff(Box::new(Some(&data))).await;

                match serve_valid{
                    Ok(_)=>{
                        self.update_staff_match(&data, id, recv_timestamp).await
                    }
                    Err(e)=> return Err(StaffError("error in staff validation".to_string()))
                }
            }
            CommandsStaff::CreateAddress {
                id,
                data,
                recv_timestamp
            } => {
                let serve_valid = service.validate_address(Box::new(Some(&data))).await;

                match serve_valid{
                    Ok(_)=>{
                        self.create_address_match(&data, id, recv_timestamp).await
                    }
                    Err(e)=> return Err(StaffError("error in address validation".to_string()))
                }
            }

            CommandsStaff::UpdateAddress {
                recv_timestamp,
                data,
                id
            } => {
                let serve_valid = service.validate_address(Box::new(Some(&data))).await;

                match serve_valid{
                    Ok(_)=>{
                        self.update_address_match(&data, id, recv_timestamp).await
                    }
                    Err(e)=> return Err(StaffError("error in address validation".to_string()))
                }
            }
            CommandsStaff::CreateContact {
                id,
                data,
                recv_timestamp
            } => {
                let serve_valid = service.validate_contact(Box::new(Some(&data))).await;

                match serve_valid{
                    Ok(_)=>{
                        self.create_contact_match(&data, id, recv_timestamp).await
                    }
                    Err(e)=> return Err(StaffError("error in contact validation".to_string()))
                }
            }
            CommandsStaff::UpdateContact {
                id,
                data,
                recv_timestamp
            } => {
                let serve_valid = service.validate_contact(Box::new(Some(&data))).await;

                match serve_valid{
                    Ok(_)=>{
                        self.update_contact_match(&data, id, recv_timestamp).await
                    }
                    Err(e)=> return Err(StaffError("error in contact validation".to_string()))
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