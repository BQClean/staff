use async_trait::async_trait;
use crate::domain::commands::{CmdAddress, CmdContact, CmdStaff};
use crate::domain::events::StaffError;
use crate::traits::service_staff::IStaffService;
use validator::{Validate, ValidationError};

pub struct StaffService {}

impl StaffService{
    pub fn new()-> Box<dyn IStaffService> {
        let x = Box::new(StaffService {});
        return x
    }
}

#[async_trait]
impl IStaffService for StaffService {
    async fn validate_staff(&self, command: Box<Option<&CmdStaff>>)
        -> Result<(), StaffError> {
        let staff= command.as_ref();
        match staff {
            Some(ref stf) => {
                match stf.validate() {
                    Ok(_) => (),
                    Err(e) => return Err(StaffError("error in staff validation".to_string())),
                }
            }
            None => {
                return Err(StaffError("staff is not defined".to_string()))
            }
        }

        Ok(())
    }

    async fn validate_address(&self, command: Box<Option<&CmdAddress>>)
        -> Result<(), StaffError> {
        let address= command.as_ref();
        match address {
            Some(ref address)=>{
                match address.validate() {
                    Ok(_)=>(),
                    Err(e)=> return Err(StaffError("address is in invalid state".to_string()))
                }
            }
            None=>{
                return Err(StaffError("address is not defined".to_string()))
            }
        }

        Ok(())
    }

    async fn validate_contact(&self, command: Box<Option<&CmdContact>>)
        -> Result<(), StaffError> {
       let contacts =command.as_ref();
        match contacts {
            Some(ref cont )=>{
                match cont.validate() {
                    Ok(_)=>(),
                    Err(e)=> return Err(StaffError("contact is  in invalid state".to_string()))
                }
            }
            None=>{
                return Err(StaffError("contact is not defined".to_string()))
            }
        }

        Ok(())
    }
}