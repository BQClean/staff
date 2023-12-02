use serde::{Deserialize, Serialize};

pub enum  EntityType {
     EntStaff,
     EntAddress,
     EntContact
 }

 #[derive(Deserialize,Serialize,Clone,PartialEq,Debug)]
 pub enum Operation{
     On,
     Off
 }