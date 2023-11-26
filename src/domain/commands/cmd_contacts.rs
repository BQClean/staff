use serde::Deserialize;

#[derive(Deserialize,Clone)]
pub struct Contact{
    id:String,
    contact_type_id:String,
    contact_value:String
}