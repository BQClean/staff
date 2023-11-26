use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Serialize,Deserialize,PartialEq)]
pub enum StaffEvent{
    StaffCreated{
        corelation_id:String,
        data:Staff,
        recv_timestamp: DateTime<Utc>,
    }
}