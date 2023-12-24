use crate::domain::common::{AddressIn, ContactIn, StaffIn};

#[derive(Deserialize, Serialize, Clone)]
pub struct AggStaff {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub vehicle_reg: String,
    pub driver_license: String,
    pub in_contract: bool,
    pub active: bool,
    pub address: Vec<AggAddress>,
    pub contacts: Vec<AggContact>,
    pub operation:Operation,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct AggContact {
    pub id: String,
    pub contact_type_id: String,
    pub contact_value: String,
    pub staff_id:String,
    pub primary:bool,
    pub operation:Operation
}
#[derive(Deserialize, Serialize, Clone)]
pub struct AggAddress {
    pub id: String,
    pub street: String,
    pub state: String,
    pub post_code: String,
    pub country: String,
    pub staff_id: String,
    pub primary:bool,
    pub operation:Operation
}


impl StaffIn for AggStaff{
    fn id(&self) -> &str {return &self.id }
    fn first_name(&self) -> &str {return &self.first_name}
    fn last_name(&self) -> &str {return &self.last_name}
    fn vehicle_reg(&self) -> &str {return &self.vehicle_reg}
    fn driver_license(&self) -> &str {return &self.driver_license }
    fn in_contract(&self) -> bool {return self.in_contract }
    fn active(&self) -> bool {return self.active}
}

impl AddressIn for AggAddress{
    fn id(&self) -> &str {return &self.id}
    fn street(&self) -> &str {return &self.street}
    fn state(&self) -> &str {&self.state}
    fn post_code(&self) -> &str {return &self.post_code}
    fn country(&self) -> &str {return &self.country}
    fn staff_id(&self) -> &str {return &self.staff_id}
    fn primary(&self) -> bool {return self.primary}
}

impl ContactIn for AggContact{
    fn id(&self) -> &str {return &self.id}
    fn contact_type_id(&self) -> &str {return &self.contact_type_id}
    fn contact_value(&self) -> &str {return &self.contact_value}
    fn staff_id(&self) -> &str {return &self.staff_id}
    fn primary(&self) -> bool {return self.primary}
}


impl Default for AggStaff {
    fn default() -> Self {
        AggStaff{
            id:"".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            vehicle_reg: "".to_string(),
            driver_license: "".to_string(),
            in_contract: false,
            active: false,
            address: vec![],
            contacts: vec![],
            operation: Operation::On,
        }
    }
}
impl Default for AggAddress {
    fn default() -> Self {
        AggAddress{
            id: "".to_string(),
            street: "".to_string(),
            state: "".to_string(),
            post_code: "".to_string(),
            country: "".to_string(),
            staff_id: "".to_string(),
            primary: false,
            operation: Operation::On,
        }
    }
}
impl Default for AggContact{
    fn default() -> Self {
        AggContact{
            id: "".to_string(),
            contact_type_id: "".to_string(),
            contact_value: "".to_string(),
            staff_id: "".to_string(),
            primary: false,
            operation: Operation::On,
        }
    }
}