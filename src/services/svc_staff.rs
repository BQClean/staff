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
        todo!()
    }

    async fn validate_address(&self, command: Box<Option<CmdAddress>>) -> Result<(), StaffError> {
        todo!()
    }

    async fn validate_contact(&self, command: Box<Option<CmdContact>>) -> Result<(), StaffError> {
        todo!()
    }
}