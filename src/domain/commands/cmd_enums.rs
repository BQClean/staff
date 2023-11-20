use std::process::id;

pub struct Command<T>{
    id:String,
    data:Option<T>
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
    pub fn  create_staff(staff:Staff)-> Commands{
        return Commands::CreateStaff(Command{
            id:CMD_CREATE_STAFF,
            data:Some(staff)
        })
    }
    pub fn  update_staff(staff:Staff)-> Commands{
        return Commands::UpdateStaff(Command{
            id:CMD_UPDATE_STAFF,
            data:Some(staff)
        })
    }
    pub fn  inactivate_staff(staff:StaffActive)-> Commands{
        return Commands::InactiveStaff(Command{
            id:CMD_ACTIVE_STAFF,
            data:Some(staff)
        })
    }
    pub fn  create_address(address:Address)-> Commands{
        return Commands::CreateAddress(Command{
            id:CMD_CREATE_ADDRESS,
            data:Some(address)
        })
    }
    pub fn  update_address(address:Address)-> Commands{
        return Commands::UpdateAddress(Command{
            id:CMD_UPDATE_ADDRESS,
            data:Some(address)
        })
    }

    pub fn  create_contact(contract: Contract)-> Commands{
        return Commands::CreateContact(Command{
            id:CMD_CREATE_CONTACT,
            data:Some(contract)
        })
    }

    pub fn  update_contact(contract: Contract)-> Commands{
        return Commands::UpdateContact(Command{
            id:CMD_UPDATE_CONTACT,
            data:Some(contract)
        })
    }
}