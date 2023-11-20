#[derive(Deserialize,Copy, Clone)]
pub struct Staff {
    id: String,
    first_name:String,
    last_name:String,
    vehicle_reg:String,
    driver_license:String,
    in_contract:bool
}

#[derive(Deserialize,Copy, Clone)]
pub struct  StaffActive{
    id: String,
    active:bool
}