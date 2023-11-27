#[derive(Deserialize,Serialize,Clone)]
pub struct Staff{
   pub id: String,
   pub first_name:String,
   pub last_name:String,
   pub vehicle_reg:String,
   pub driver_license:String,
   pub in_contract:bool
}

#[derive(Deserialize,Serialize,Clone)]
pub struct  StaffActive{
   pub id: String,
   pub active:bool
}