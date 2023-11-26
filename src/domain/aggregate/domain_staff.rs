use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};
use crate::domain::commands::CommandsStaff;
use crate::domain::entities::{EntityAddress, EntityContract};

#[derive(Deserialize,Serialize,Clone)]
pub struct RootStaff{
    id: uuid::Uuid,
    first_name:String,
    last_name:String,
    vehicle_reg:String,
    driver_license:String,
    in_contract:bool,
    active:bool,
    address:Vec<Contract>,
    contacts:Vec<Address>
}
#[derive(Deserialize,Serialize,Clone)]
pub struct Contract{
    id:String,
    contact_type_id:String,
    contact_value:String
}
#[derive(Deserialize,Serialize,Clone)]
pub struct  Address{
    id:String,
    street:String,
    state:String,
    post_code:String,
    country:String,
    staff_id:String
}

#[async_trait]
impl Aggregate for RootStaff{
    type Command = CommandsStaff;
    type Event = ();
    type Error = ();
    type Services = ();

    fn aggregate_type() -> String {
       return String::from("staff")
    }

    async fn handle(&self,
                    command: Self::Command,
                    service: &Self::Services) -> Result<Vec<Self::Event>, Self::Error> {
        todo!()
    }

    fn apply(&mut self, event: Self::Event) {
        todo!()
    }
}

impl Default for RootStaff {
    fn default() -> Self {
        todo!()
    }
}