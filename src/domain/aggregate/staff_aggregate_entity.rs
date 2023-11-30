
#[derive(Deserialize, Serialize, Clone)]
pub struct AggStaff {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub vehicle_reg: String,
    pub driver_license: String,
    pub in_contract: bool,
    pub active: bool,
    pub address: Vec<AggAddress>,
    pub contacts: Vec<AggContact>,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct AggContact {
    pub id: String,
    pub contact_type_id: String,
    pub contact_value: String,
    pub staff_id:String,
    pub primary:bool
}
#[derive(Deserialize, Serialize, Clone)]
pub struct AggAddress {
    pub id: String,
    pub street: String,
    pub state: String,
    pub post_code: String,
    pub country: String,
    pub staff_id: String,
    pub primary:bool
}