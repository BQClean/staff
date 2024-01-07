use std::sync::Arc;
use postgres_es::PostgresCqrs;
use crate::domain::aggregate::AggStaff;

#[derive(Clone)]
pub struct ServiceState{
    pub cqrs:Arc<PostgresCqrs<AggStaff>>
}


impl ServiceState {
    pub fn new()->Self{

    }
}