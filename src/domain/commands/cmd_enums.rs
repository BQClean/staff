
use std::process::id;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;
use crate::helpers as hp;

#[derive(Serialize,Deserialize,Default)]
pub struct Command<T>{
    id:String,
    data:Option<T>,
    correlation_id: Option<String>,
    recv_timestamp: DateTime<Utc>,
}

pub enum  Commands {
    CreateStaff(Command<Staff>),
    UpdateStaff(Command<Staff>),
    InactiveStaff(Command<StaffActive>),
    CreateAddress(Command<Address>),
    UpdateAddress(Command<Address>),
    CreateContact(Command<Contract>),
    UpdateContact(Command<Contract>)
}

impl Commands{
    pub fn  create_staff(staff:Staff,coreid: Option<String>)-> Commands{
        return Commands::CreateStaff(Command{
            id:CMD_CREATE_STAFF.to_string(),
            data:Some(staff),
            correlation_id:coreid,
            recv_timestamp:chrono::offset::Utc::now()
        });

    }
    pub fn  update_staff(staff:Staff,co_relation_id: Option<String>)-> Commands{
        return Commands::UpdateStaff(Command{
            id:CMD_UPDATE_STAFF.to_string(),
            data:Some(staff),
            correlation_id:co_relation_id,
            recv_timestamp:chrono::offset::Utc::now()
        })
    }
    pub fn  inactivate_staff(staff:StaffActive,coreid: Option<String>)-> Commands{
        return Commands::InactiveStaff(Command{
            id:CMD_ACTIVE_STAFF.to_string(),
            data:Some(staff),
            correlation_id:coreid,
            recv_timestamp:chrono::offset::Utc::now()

        })
    }
    pub fn  create_address(address:Address,coreid: Option<String>)-> Commands{
        return Commands::CreateAddress(Command{
            id:CMD_CREATE_ADDRESS.to_string(),
            data:Some(address),
            correlation_id:coreid,
            recv_timestamp:chrono::offset::Utc::now()
        })
    }
    pub fn  update_address(address:Address,coreid: Option<String>)-> Commands{
        return Commands::UpdateAddress(Command{
            id:CMD_UPDATE_ADDRESS.to_string(),
            data:Some(address),
            correlation_id:coreid,
            recv_timestamp:chrono::offset::Utc::now()
        })
    }

    pub fn  create_contact(contract: Contract,coreid: Option<String>)-> Commands{
        return Commands::CreateContact(Command{
            id:CMD_CREATE_CONTACT.to_string(),
            data:Some(contract),
            correlation_id:coreid,
            recv_timestamp:chrono::offset::Utc::now()
        })
    }

    pub fn  update_contact(contract: Contract,coreid: Option<String>)-> Commands{
        return Commands::UpdateContact(Command{
            id:CMD_UPDATE_CONTACT.to_string(),
            data:Some(contract),
            correlation_id:coreid,
            recv_timestamp:chrono::offset::Utc::now()
        })
    }
}