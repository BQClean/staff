
use std::process::id;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;
use crate::helpers as hp;

#[derive(Serialize,Deserialize)]
pub enum CommandsStaff {
    CreateStaff{
        id:String,
        recv_timestamp: DateTime<Utc>,
        data:Staff
    },
    UpdateStaff{
        id:String,
        recv_timestamp: DateTime<Utc>,
        data:Staff
    },
    InactiveStaff{
        id:String,
        recv_timestamp: DateTime<Utc>,
        data:StaffActive
    },
    CreateAddress{
        id:String,
        recv_timestamp: DateTime<Utc>,
        data:Address
    },
    UpdateAddress{
        id:String,
        recv_timestamp: DateTime<Utc>,
        data:Address
    },
    CreateContact{
        id:String,
        recv_timestamp: DateTime<Utc>,
        data:Contact
    },
    UpdateContact{
        id:String,
        recv_timestamp: DateTime<Utc>,
        data:Contact
    }
}

impl CommandsStaff {
    pub fn  create_staff(staff:Staff)-> CommandsStaff {
        return CommandsStaff::CreateStaff{
            id:CMD_CREATE_STAFF.to_string(),
            recv_timestamp:chrono::offset::Utc::now(),
            data:staff
        };
    }
    pub fn  update_staff(staff:Staff)-> CommandsStaff {
        return CommandsStaff::UpdateStaff{
            id:CMD_UPDATE_STAFF.to_string(),
            data:staff,
            recv_timestamp:chrono::offset::Utc::now()
        }
    }
    pub fn  inactivate_staff(staff:StaffActive)-> CommandsStaff {
        return CommandsStaff::InactiveStaff{
            id:CMD_ACTIVE_STAFF.to_string(),
            data:staff,
            recv_timestamp:chrono::offset::Utc::now()

        }
    }
    pub fn  create_address(address:Address)-> CommandsStaff {
        return CommandsStaff::CreateAddress{
            id:CMD_CREATE_ADDRESS.to_string(),
            data:address,
            recv_timestamp:chrono::offset::Utc::now()
        }
    }
    pub fn  update_address(address:Address)-> CommandsStaff {
        return CommandsStaff::UpdateAddress{
            id:CMD_UPDATE_ADDRESS.to_string(),
            data:address,
            recv_timestamp:chrono::offset::Utc::now()
        }
    }
    pub fn  create_contact(contract: Contact)-> CommandsStaff {
        return CommandsStaff::CreateContact{
            id:CMD_CREATE_CONTACT.to_string(),
            data:contract,
            recv_timestamp:chrono::offset::Utc::now()
        }
    }
    pub fn  update_contact(contact: Contact)-> CommandsStaff {
        return CommandsStaff::UpdateContact{
            id:CMD_UPDATE_CONTACT.to_string(),
            data:contact,
            recv_timestamp:chrono::offset::Utc::now()
        }
    }
}