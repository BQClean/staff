use serde::Deserialize;

#[derive(Deserialize,Clone)]
pub struct Contract{
    id:String,
    contact_type_id:String,
    contact_value:String
}