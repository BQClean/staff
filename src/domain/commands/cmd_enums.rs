
pub enum  Commands {
    CreateStaff{
        id:String,
        data:Staff
    },
    UpdateStaff{
        id:String,
        data:Staff
    },
    InactiveStaff{
        id:String,
        data:StaffActive
    },
    CreateAddress{
        id:String,
        data:Address
    },
    UpdateAddress{
        id:String,
        data:Address
    }
}