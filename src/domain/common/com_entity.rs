
pub trait StaffIn {
    fn id(&self) -> &str;
    fn first_name(&self) -> &str;
    fn last_name(&self) -> &str;
    fn vehicle_reg(&self) -> &str;
    fn driver_license(&self) -> &str;
    fn in_contract(&self) -> bool;
    fn active(&self) ->bool;
}


pub trait ContactIn {
    fn id(&self) -> &str;
    fn contact_type_id(&self) -> &str;
    fn contact_value(&self) -> &str;
    fn staff_id(&self)-> &str;
    fn primary(&self)-> bool;
}

pub trait AddressIn {
    fn id(&self) -> &str;
    fn street(&self)-> &str;
    fn state(&self)->&str;
    fn post_code(&self)-> &str;
    fn country(&self) ->&str;
    fn staff_id(&self)->&str;
    fn primary(&self)->bool;
}

pub trait StaffActiveIn{
    fn id(&self) -> &str;
    fn active(&self) ->bool;
}