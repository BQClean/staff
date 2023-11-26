use serde::Deserialize;

#[derive(Deserialize,Serialize,Clone)]
pub struct EntityContract{
    id:String,
    contact_type_id:String,
    contact_value:String
}