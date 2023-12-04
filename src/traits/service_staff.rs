use async_trait::async_trait;
use crate::domain::commands::{CmdAddress, CmdContact};

#[async_trait]
pub trait  IStaffService:Sync + Send {
    async fn validate_staff(&self,command:Box<Option<CmdStaff>>)->Result<(),StaffError>;
    async fn validate_address(&self,command:Box<Option<CmdAddress>>)->Result<(),StaffError>;
    async fn validate_contact(&self,command:Box<Option<CmdContact>>)->Result<(),StaffError>;
}