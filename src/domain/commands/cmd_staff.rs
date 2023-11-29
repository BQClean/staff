use serde::Deserialize;

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
pub struct  Address{
   pub id:String,
   pub street:String,
   pub  state:String,
   pub post_code:String,
   pub country:String,
   pub staff_id:String
}

#[derive(Deserialize,Serialize,Clone)]
pub struct Contact{
   pub id:String,
   pub contact_type_id:String,
   pub contact_value:String
}

#[derive(Deserialize,Serialize,Clone)]
pub struct  StaffActive{
   pub id: String,
   pub active:bool
}

