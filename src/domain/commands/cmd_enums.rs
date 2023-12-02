
use std::process::id;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;
use crate::helpers as hp;

#[derive(Serialize,Deserialize)]
pub enum CommandsStaff {
    CreateStaff{
        id:String,
        recv_timestamp: DateTime<Utc>,
        data: CmdStaff
    },
    UpdateStaff{
        id:String,
        recv_timestamp: DateTime<Utc>,
        data: CmdStaff
    },
    CreateAddress{
        id:String,
        recv_timestamp: DateTime<Utc>,
        data: CmdAddress
    },
    UpdateAddress{
        id:String,
        recv_timestamp: DateTime<Utc>,
        data: CmdAddress
    },
    CreateContact{
        id:String,
        recv_timestamp: DateTime<Utc>,
        data: CmdContact
    },
    UpdateContact{
        id:String,
        recv_timestamp: DateTime<Utc>,
        data: CmdContact
    }
}

impl CommandsStaff {
    pub fn  create_staff(staff: CmdStaff) -> CommandsStaff {
        return CommandsStaff::CreateStaff{
            id:CMD_CREATE_STAFF.to_string(),
            recv_timestamp:chrono::offset::Utc::now(),
            data:staff
        };
    }
    pub fn  update_staff(staff: CmdStaff) -> CommandsStaff {
        return CommandsStaff::UpdateStaff{
            id:CMD_UPDATE_STAFF.to_string(),
            data:staff,
            recv_timestamp:chrono::offset::Utc::now()
        }
    }
    pub fn  create_address(address: CmdAddress) -> CommandsStaff {
        return CommandsStaff::CreateAddress{
            id:CMD_CREATE_ADDRESS.to_string(),
            data:address,
            recv_timestamp:chrono::offset::Utc::now()
        }
    }
    pub fn  update_address(address: CmdAddress) -> CommandsStaff {
        return CommandsStaff::UpdateAddress{
            id:CMD_UPDATE_ADDRESS.to_string(),
            data:address,
            recv_timestamp:chrono::offset::Utc::now()
        }
    }
    pub fn  create_contact(contract: CmdContact) -> CommandsStaff {
        return CommandsStaff::CreateContact{
            id:CMD_CREATE_CONTACT.to_string(),
            data:contract,
            recv_timestamp:chrono::offset::Utc::now()
        }
    }
    pub fn  update_contact(contact: CmdContact) -> CommandsStaff {
        return CommandsStaff::UpdateContact{
            id:CMD_UPDATE_CONTACT.to_string(),
            data:contact,
            recv_timestamp:chrono::offset::Utc::now()
        }
    }
}