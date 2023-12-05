use async_trait::async_trait;
use crate::domain::commands::{CmdAddress, CmdContact, CmdStaff};
use crate::domain::events::StaffError;
use crate::traits::IStaffService;

pub struct StaffService {}

impl StaffService{
    pub fn new()-> Box<dyn IStaffService> {
        let x = Box::new(StaffService {});
        return x
    }
}

#[async_trait]
impl IStaffService for StaffService {
    async fn validate_staff(&self, command: Box<Option<CmdStaff>>) -> Result<(), StaffError> {

        let staff= command.as_ref();
        match staff {
            Some(ref stf)=>{
                if stf.id.is_empty(){
                    return Err(StaffError("id is not defined".to_string()))
                }

                if stf.first_name.is_empty(){
                    return Err(StaffError("first name is not defined".to_string()))
                }

                if stf.last_name.is_empty(){
                    return Err(StaffError("last name is not defined".to_string()))
                }

                if stf.driver_license.is_empty(){
                    return Err(StaffError("driver licence is not defined".to_string()))
                }
            }
            None=>{
               return Err(StaffError("staff is not defined".to_string()))
            }
        }

       Ok(())
    }

    async fn validate_address(&self, command: Box<Option<CmdAddress>>) -> Result<(), StaffError> {
        todo!()
    }

    async fn validate_contact(&self, command: Box<Option<CmdContact>>) -> Result<(), StaffError> {
        todo!()
    }
}