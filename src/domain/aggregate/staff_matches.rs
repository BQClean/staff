use chrono::{DateTime, Utc};

impl AggStaff {
    /// Creates a new staff with the given data.
    ///
    /// # Arguments
    ///
    /// * `data` - The command to create the staff.
    /// * `id` - The ID of the staff to be created.
    /// * `receive_timestamp` - The receive timestamp of the staff creation request.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `StaffEvent` if the staff was created successfully,
    /// or a `StaffError` if an error occurred during staff creation
    pub(crate) async fn create_staff_match(&self,
                                           data: CmdStaff,
                                           id: String,
                                           recv_timestamp: DateTime<Utc>) -> Result<Vec<StaffEvent>, StaffError> {
        let staff_val = self.get_staff_event(Box::new(Some(data)));

        match staff_val {
            Some(staff) => {
                Ok(vec![StaffEvent::StaffCreated(CommonEvent {
                    corelation_id: id,
                    data: staff,
                    recv_timestamp,
                })])
            }
            None => {
                Err(StaffError("error in processing staff command".to_string()))
            }
        }
    }


    /// Updates a staff match with the provided data.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to update the staff match with.
    /// * `id` - The corelation ID for the staff update event.
    /// * `recv_timestamp` - The timestamp when the staff update event was received.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing a vector of [`StaffEvent`]s if the staff match was updated successfully,
    /// or a [`StaffError`] if there was an error in processing the staff command.
    pub(crate) async fn update_staff_match(&self,
                                           data: CmdStaff,
                                           id: String,
                                           recv_timestamp: DateTime<Utc>) -> Result<Vec<StaffEvent>, StaffError> {
        let staff_val = self.get_staff_event(Box::new(Some(data)));
        match staff_val {
            Some(staff) => {
                Ok(vec![StaffEvent::StaffUpdated(CommonEvent {
                    corelation_id: id,
                    data: staff,
                    recv_timestamp,
                })])
            }
            None => {
                Err(StaffError("error in processing staff command".to_string()))
            }
        }
    }


    /// Creates an address match from the provided command data.
    ///
    /// # Arguments
    ///
    /// * `data` - The command data used to create the address match.
    /// * `id` - The correlation id for the address match.
    /// * `recv_timestamp` - The timestamp when the address match was received.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<StaffEvent>, StaffError>` - A result containing
    /// either a vector of staff events if the address match was successfully created, or a `
    pub(crate) async fn create_address_match(&self,
                                             data: CmdAddress,
                                             id: String,
                                             recv_timestamp: DateTime<Utc>) -> Result<Vec<StaffEvent>, StaffError> {
        let staff_val = self.get_address_event(Box::new(Some(data)));
        match staff_val {
            Some(staff) => {
                Ok(vec![StaffEvent::AddressCreated(CommonEvent {
                    corelation_id: id,
                    data: staff,
                    recv_timestamp,
                })])
            }
            None => {
                Err(StaffError("error in processing address command".to_string()))
            }
        }
    }


    /// Updates the address match using the given data and returns a vector of staff events.
    ///
    /// # Arguments
    ///
    /// * `data` - A `CmdAddress` struct that represents the updated address data.
    /// * `id` - A String that represents the correlation ID.
    /// * `recv_timestamp` - A `DateTime<Utc>` that represents the timestamp when the command was received.
    ///
    /// # Returns
    ///
    /// Returns a `Result` that contains a vector of `StaffEvent`
    /// if the address update is successful. Otherwise, returns a `StaffError` indicating an error
    pub(crate) async fn update_address_match(&self,
                                             data: CmdAddress,
                                             id: String,
                                             recv_timestamp: DateTime<Utc>) -> Result<Vec<StaffEvent>, StaffError> {
        let staff_val = self.get_address_event(Box::new(Some(data)));
        match staff_val {
            Some(staff) => {
                Ok(vec![StaffEvent::AddressUpdated(CommonEvent {
                    corelation_id: id,
                    data: staff,
                    recv_timestamp,
                })])
            }
            None => {
                Err(StaffError("error in processing address command".to_string()))
            }
        }
    }


    /// Creates a contact match given the data, id, and received timestamp.
    ///
    /// # Arguments
    ///
    /// * `data` - The data for the contact to be created.
    /// * `id` - The ID of the contact match.
    /// * `recv_timestamp` - The timestamp when the contact match was received.
    ///
    /// # Returns
    ///
    /// Returns a result containing a vector of staff events if the contact match was created successfully,
    /// otherwise returns a StaffError.
    ///
    /// # Example
    ///
    /// ```
    /// use std::result::Result;
    /// use chrono::{DateTime,
    pub(crate) async fn create_contact_match(&self,
                                             data: CmdContact,
                                             id: String,
                                             recv_timestamp: DateTime<Utc>) -> Result<Vec<StaffEvent>, StaffError> {
        let staff_val = self.get_contact_event(Box::new(Some(data)));
        match staff_val {
            Some(staff) => {
                Ok(vec![StaffEvent::ContactCreated(CommonEvent {
                    corelation_id: id,
                    data: staff,
                    recv_timestamp,
                })])
            }
            None => {
                Err(StaffError("error in processing contact command".to_string()))
            }
        }
    }


    /// Updates a contact match with the given data, ID, and receive timestamp.
    ///
    /// # Arguments
    ///
    /// * `data` - The contact data to update.
    /// * `id` - The ID of the contact match to update.
    /// * `recv_timestamp` - The receive timestamp for the update.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `StaffEvent` if the contact was updated successfully,
    /// or a `StaffError` if there was an error processing the
    pub(crate) async fn update_contact_match(&self,
                                             data: CmdContact,
                                             id: String,
                                             recv_timestamp: DateTime<Utc>) -> Result<Vec<StaffEvent>, StaffError> {
        let staff_val = self.get_contact_event(Box::new(Some(data)));
        match staff_val {
            Some(staff) => {
                Ok(vec![StaffEvent::ContactUpdated(CommonEvent {
                    corelation_id: id,
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