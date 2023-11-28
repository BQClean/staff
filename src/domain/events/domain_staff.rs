
#[derive(Deserialize,Serialize, Clone,PartialEq,Debug)]
pub struct EventStaff {
    pub id: String,
    pub first_name:String,
    pub last_name:String,
    pub vehicle_reg:String,
    pub driver_license:String,
    pub in_contract:bool,
    pub active:bool,
    pub address:Vec<EventAddress>,
    pub contacts:Vec<EventContact>
}
#[derive(Deserialize,Serialize, Clone,PartialEq,Debug)]
pub struct EventContact{
    pub id:String,
    pub contact_type_id:String,
    pub contact_value:String,
    pub staff_id:String,
    pub primary:bool
}
#[derive(Deserialize,Serialize, Clone,PartialEq,Debug)]
pub struct  EventAddress{
    pub id:String,
    pub street:String,
    pub state:String,
    pub post_code:String,
    pub country:String,
    pub staff_id:String,
    pub primary:bool
}