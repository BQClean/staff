use serde::Deserialize;

#[derive(Deserialize,Serialize,Clone)]
pub struct Contact{
    pub id:String,
    pub contact_type_id:String,
    pub contact_value:String
}