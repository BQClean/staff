use async_trait::async_trait;
use cqrs_es::Aggregate;
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

#[async_trait]
impl Aggregate for StaffAggregateEntity{
    type Command = ();
    type Event = ();
    type Error = ();
    type Services = ();

    fn aggregate_type() -> String {
        todo!()
    }

    async fn handle(&self, command: Self::Command, service: &Self::Services) -> Result<Vec<Self::Event>, Self::Error> {
        todo!()
    }

    fn apply(&mut self, event: Self::Event) {
        todo!()
    }
}

impl Default for StaffAggregateEntity {
    fn default() -> Self {
        todo!()
    }
}