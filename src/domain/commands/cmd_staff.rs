use serde::Deserialize;
use crate::domain::common;
use crate::domain::common::{AddressIn, ContactIn, StaffActiveIn, StaffIn};

#[derive(Deserialize,Serialize,Clone)]
pub struct CmdStaff {
   pub id: String,
   pub first_name:String,
   pub last_name:String,
   pub vehicle_reg:String,
   pub driver_license:String,
   pub in_contract:bool,
   pub active:bool
}

#[derive(Deserialize,Serialize,Clone)]
pub struct CmdAddress {
   pub id:String,
   pub street:String,
   pub state:String,
   pub post_code:String,
   pub country:String,
   pub staff_id:String,
   pub primary:bool
}

#[derive(Deserialize,Serialize,Clone)]
pub struct CmdContact {
   pub id:String,
   pub contact_type_id:String,
   pub contact_value:String,
   pub staff_id:String,
   pub primary:bool
}

#[derive(Deserialize,Serialize,Clone)]
pub struct  CmdStaffActive{
   pub id: String,
   pub active:bool
}

impl StaffIn for CmdStaff {
   fn id(&self) -> &str {return &self.id}
   fn first_name(&self) -> &str {return &self.first_name}
   fn last_name(&self) -> &str {return &self.last_name}
   fn vehicle_reg(&self) -> &str {return &self.vehicle_reg}
   fn driver_license(&self) -> &str {return &self.driver_license}
   fn in_contract(&self) -> bool {return self.in_contract}
   fn active(&self) -> bool {return self.active }
}

impl AddressIn for CmdAddress {
   fn id(&self) -> &str {return &self.id}
   fn street(&self) -> &str {return &self.street}
   fn state(&self) -> &str {&self.state}
   fn post_code(&self) -> &str {return &self.post_code}
   fn country(&self) -> &str {return &self.country}
   fn staff_id(&self) -> &str {return &self.staff_id}
   fn primary(&self) -> bool {return self.primary}
}

impl ContactIn for CmdContact {
   fn id(&self) -> &str {return &self.id}
   fn contact_type_id(&self) -> &str {return &self.contact_type_id}
   fn contact_value(&self) -> &str {return &self.contact_value}
   fn staff_id(&self) -> &str {return &self.staff_id}
   fn primary(&self) -> bool {return self.primary}
}

impl StaffActiveIn for CmdStaffActive{
   fn id(&self) -> &str {return &self.id}
   fn active(&self) -> bool {return self.active}
}