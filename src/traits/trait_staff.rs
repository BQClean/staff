use async_trait::async_trait;
use comentities::staff::commands::{CmdRootStaff, CmdStaffAddress, CmdStaffContact};
use crate::domain::events::StaffError;

#[async_trait]
pub trait  IStaffService:Sync + Send {
    async fn validate_staff(&self,command:Box<Option<&CmdRootStaff>>)->Result<(),StaffError>;
    async fn validate_address(&self,command:Box<Option<&CmdStaffAddress>>)->Result<(),StaffError>;
    async fn validate_contact(&self,command:Box<Option<&CmdStaffContact>>)->Result<(),StaffError>;
}