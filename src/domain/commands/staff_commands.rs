use serde::Deserialize;
use crate::domain::common;
use crate::domain::common::{AddressIn, ContactIn, StaffActiveIn, StaffIn};
use validator::{Validate, ValidationError};
use comentities::staff::commands::{CmdRootStaff,CmdStaffActive,CmdStaffContact,CmdStaffAddress};

impl StaffIn for CmdRootStaff {
   fn id(&self) -> &str {return &self.id}
   fn first_name(&self) -> &str {return &self.first_name}
   fn last_name(&self) -> &str {return &self.last_name}
   fn vehicle_reg(&self) -> &str {return &self.vehicle_reg}
   fn driver_license(&self) -> &str {return &self.driver_license}
   fn in_contract(&self) -> bool {return self.in_contract}
   fn active(&self) -> bool {return self.active }
}

impl AddressIn for CmdStaffAddress {
   fn id(&self) -> &str {return &self.id}
   fn street(&self) -> &str {return &self.street}
   fn state(&self) -> &str {&self.state}
   fn post_code(&self) -> &str {return &self.post_code}
   fn country(&self) -> &str {return &self.country}
   fn staff_id(&self) -> &str {return &self.staff_id}
   fn primary(&self) -> bool {return self.primary}
}

impl ContactIn for CmdStaffContact {
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