
#[derive(Deserialize,Clone)]
pub struct Staff {
    id: String,
    first_name:String,
    last_name:String,
    vehicle_reg:String,
    driver_license:String,
    in_contract:bool,
    active:bool,
    address:Vec<Contract>,
    contacts:Vec<Address>
}

#[derive(Deserialize,Clone)]
pub struct Contract{
    id:String,
    contact_type_id:String,
    contact_value:String
}
#[derive(Deserialize,Clone)]
pub struct  Address{
    id:String,
    street:String,
    state:String,
    post_code:String,
    country:String,
    staff_id:String
}