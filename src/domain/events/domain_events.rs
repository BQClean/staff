use std::fmt::{Display, Formatter};
use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use comentities::staff::events::EventRootStaff;
use comentities::common::com_event::CommonEvent;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StaffEvent {
    StaffCreated(CommonEvent<EventRootStaff>),
    StaffUpdated(CommonEvent<EventRootStaff>),
    AddressCreated(CommonEvent<EventRootStaff>),
    AddressUpdated(CommonEvent<EventRootStaff>),
    ContactCreated(CommonEvent<EventRootStaff>),
    ContactUpdated(CommonEvent<EventRootStaff>),
}

impl DomainEvent for StaffEvent {
    fn event_type(&self) -> String {
        match self {
            StaffEvent::StaffCreated { .. } => "StaffCreated".to_string(),
            StaffEvent::StaffUpdated { .. } => "StaffUpdated".to_string(),
            StaffEvent::AddressCreated { .. } => "AddressCreated".to_string(),
            StaffEvent::AddressUpdated { .. } => "AddressUpdated".to_string(),
            StaffEvent::ContactCreated { .. } => "ContactCreated".to_string(),
            StaffEvent::ContactUpdated { .. } => "ContactUpdated".to_string(),
        }
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug)]
pub struct StaffError(pub String);

impl From<&str> for StaffError {
    fn from(value: &str) -> Self {
        return Self(value.to_string());
    }
}

impl Display for StaffError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.0);
    }
}

impl std::error::Error for StaffError {}