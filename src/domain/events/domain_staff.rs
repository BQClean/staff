
#[derive(Deserialize,Serialize, Clone,PartialEq)]
pub struct Staff {
    id: String,
    first_name:String,
    last_name:String,
    vehicle_reg:String,
    driver_license:String,
    in_contract:bool,
    active:bool,
    address:Vec<Contact>,
    contacts:Vec<Address>
}
#[derive(Deserialize,Serialize, Clone,PartialEq)]
pub struct Contact{
    id:String,
    contact_type_id:String,
    contact_value:String
}
#[derive(Deserialize,Serialize, Clone,PartialEq)]
pub struct  Address{
    id:String,
    street:String,
    state:String,
    post_code:String,
    country:String,
    staff_id:String
}