
pub struct  CreateAddressCommand{
    id:uuid::Uuid,
    street:String,
    state:String,
    post_code:String,
    country:String,
    staff_id:String
}

pub struct  UpdateAddressCommand{
    id:uuid::Uuid,
    street:String,
    state:String,
    post_code:String,
    country:String,
    staff_id:String
}