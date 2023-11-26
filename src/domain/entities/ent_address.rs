use serde::Serialize;

#[derive(Deserialize,Serialize,Clone)]
pub struct  EntityAddress{
    id:String,
    street:String,
    state:String,
    post_code:String,
    country:String,
    staff_id:String
}