pub struct CreateStaffCommand {
    id: uuid::Uuid,
    first_name:String,
    last_name:String,
    vehicle_reg:String,
    driver_license:String,
    in_contract:bool
}
pub struct  UpdateStaffCommand{
    id: uuid::Uuid,
    first_name:String,
    last_name:String,
    vehicle_reg:String,
    driver_license:String,
    in_contract:bool
}
pub struct  InactivateStaffCommand{
    id: uuid::Uuid,
}
