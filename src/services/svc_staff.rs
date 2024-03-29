use async_trait::async_trait;
use comentities::staff::commands::{CmdStaffAddress, CmdStaffContact, CmdRootStaff};
use crate::domain::events::StaffError;
use crate::traits::trait_staff::IStaffService;
use validator::{Validate};

pub struct StaffService {}

impl StaffService{
    pub fn new()-> Box<Self> {
        let x = Box::new(StaffService {});
        return x
    }
}

#[async_trait]
impl IStaffService for StaffService {
    async fn validate_staff(&self, command: Box<Option<&CmdRootStaff>>)
        -> Result<(), StaffError> {
        let staff= command.as_ref();
        match staff {
            Some(ref stf) => {
                match stf.validate() {
                    Ok(_) => (),
                    Err(_e) => return Err(StaffError("error in staff validation".to_string())),
                }
            }
            None => {
                return Err(StaffError("staff is not defined".to_string()))
            }
        }

        Ok(())
    }

    async fn validate_address(&self, command: Box<Option<&CmdStaffAddress>>)
        -> Result<(), StaffError> {
        let address= command.as_ref();
        match address {
            Some(ref address)=>{
                match address.validate() {
                    Ok(_)=>(),
                    Err(_e)=> return Err(StaffError("address is in invalid state".to_string()))
                }
            }
            None=>{
                return Err(StaffError("address is not defined".to_string()))
            }
        }

        Ok(())
    }

    async fn validate_contact(&self, command: Box<Option<&CmdStaffContact>>)
        -> Result<(), StaffError> {
       let contacts =command.as_ref();
        match contacts {
            Some(ref cont )=>{
                match cont.validate() {
                    Ok(_)=>(),
                    Err(_e)=> return Err(StaffError("contact is  in invalid state".to_string()))
                }
            }
            None=>{
                return Err(StaffError("contact is not defined".to_string()))
            }
        }

        Ok(())
    }
}