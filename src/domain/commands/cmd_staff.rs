pub struct Staff {
    id: uuid::Uuid,
    first_name:String,
    last_name:String,
    vehicle_reg:String,
    driver_license:String,
    in_contract:bool
}
pub struct  StaffActive{
    id: uuid::Uuid,
    active:bool
}
