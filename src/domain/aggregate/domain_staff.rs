use serde::{Deserialize, Serialize};
use crate::domain::entities::{EntityAddress, EntityContract};

#[derive(Deserialize,Serialize,Clone)]
pub struct StaffAggregateEntity{
    id: uuid::Uuid,
    first_name:String,
    last_name:String,
    vehicle_reg:String,
    driver_license:String,
    in_contract:bool,
    address:Box<Vec<EntityAddress>>,
    contacts:Box<Vec<EntityContract>>
}