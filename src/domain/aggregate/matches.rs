use chrono::{DateTime, Utc};

impl AggStaff {
   pub(crate) async fn create_staff_match(&self,
                                           data: &CmdRootStaff,
                                           id: String,
                                           recv_timestamp: DateTime<Utc>) -> Result<Vec<StaffEvent>, StaffError> {
        let staff_val = self.get_staff_event(Box::new(Some(data)));

        match staff_val {
            Some(staff) => {
                Ok(vec![StaffEvent::StaffCreated(CommonEvent {
                    co_relation_id: id,
                    data: staff,
                    recv_timestamp,
                })])
            }
            None => {
                Err(StaffError("error in processing staff command".to_string()))
            }
        }
    }
    pub(crate) async fn update_staff_match(&self,
                                           data: &CmdRootStaff,
                                           id: String,
                                           recv_timestamp: DateTime<Utc>) -> Result<Vec<StaffEvent>, StaffError> {
        let staff_val = self.get_staff_event(Box::new(Some(data)));
        match staff_val {
            Some(staff) => {
                Ok(vec![StaffEvent::StaffUpdated(CommonEvent {
                    co_relation_id: id,
                    data: staff,
                    recv_timestamp,
                })])
            }
            None => {
                Err(StaffError("error in processing staff command".to_string()))
            }
        }
    }

    pub(crate) async fn create_address_match(&self,
                                             data: &CmdStaffAddress,
                                             id: String,
                                             recv_timestamp: DateTime<Utc>) -> Result<Vec<StaffEvent>, StaffError> {
        let staff_val = self.get_address_event(Box::new(Some(data)));
        match staff_val {
            Some(staff) => {
                Ok(vec![StaffEvent::AddressCreated(CommonEvent {
                    co_relation_id: id,
                    data: staff,
                    recv_timestamp,
                })])
            }
            None => {
                Err(StaffError("error in processing address command".to_string()))
            }
        }
    }

    pub(crate) async fn update_address_match(&self,
                                             data: &CmdStaffAddress,
                                             id: String,
                                             recv_timestamp: DateTime<Utc>) -> Result<Vec<StaffEvent>, StaffError> {
        let staff_val = self.get_address_event(Box::new(Some(data)));
        match staff_val {
            Some(staff) => {
                Ok(vec![StaffEvent::AddressUpdated(CommonEvent {
                    co_relation_id: id,
                    data: staff,
                    recv_timestamp,
                })])
            }
            None => {
                Err(StaffError("error in processing address command".to_string()))
            }
        }
    }

    pub(crate) async fn create_contact_match(&self,
                                             data: &CmdStaffContact,
                                             id: String,
                                             recv_timestamp: DateTime<Utc>) -> Result<Vec<StaffEvent>, StaffError> {
        let staff_val = self.get_contact_event(Box::new(Some(data)));
        match staff_val {
            Some(staff) => {
                Ok(vec![StaffEvent::ContactCreated(CommonEvent {
                    co_relation_id: id,
                    data: staff,
                    recv_timestamp,
                })])
            }
            None => {
                Err(StaffError("error in processing contact command".to_string()))
            }
        }
    }

    pub(crate) async fn update_contact_match(&self,
                                             data: &CmdStaffContact,
                                             id: String,
                                             recv_timestamp: DateTime<Utc>) -> Result<Vec<StaffEvent>, StaffError> {
        let staff_val = self.get_contact_event(Box::new(Some(data)));
        match staff_val {
            Some(staff) => {
                Ok(vec![StaffEvent::ContactUpdated(CommonEvent {
                    co_relation_id: id,
                    data: staff,
                    recv_timestamp,
                })])
            }
            None => {
                Err(StaffError("error in processing contact command".to_string()))
            }
        }
    }
}