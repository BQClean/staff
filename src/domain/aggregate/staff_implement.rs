use std::ops::Add;
use std::option::Option;
use crate::domain::common;
use crate::domain::common::EntityType;
use std::any::Any;
use crate::domain::commands::{CmdAddress, CmdContact};

//
impl AggStaff {

    /// Retrieves the event staff based on the given command staff.
    ///
    /// # Arguments
    ///
    /// * `cmd_staff` - A boxed optional command staff object.
    ///
    /// # Returns
    ///
    /// An optional event staff object.
    pub fn get_staff_event(&self, cmd_staff: Box<Option<CmdStaff>>) -> Option<EventStaff> {
        let staff = self.compose_staff(cmd_staff, true);

        return staff;
    }

    /// Gets the event staff with the given command address.
    ///
    /// # Arguments
    ///
    /// * `cmd_address` - The command address.
    ///
    /// # Returns
    ///
    /// An optional `EventStaff` object, or `None` if the staff is not found.
    pub fn get_address_event(&self, cmd_address: Box<Option<CmdAddress>>) -> Option<EventStaff> {
        let staff = self.compose_staff(Box::new(None), true);

        let address = self.compose_address(cmd_address);

        if let Some(mut staff_opt) = staff {
            staff_opt.address = address;

            return Some(staff_opt);
        }

        return staff;
    }


    /// Retrieves a contact event for the given command contact.
    ///
    /// # Arguments
    ///
    /// * `cmd_contact` - The command contact to retrieve a contact event for.
    ///
    /// # Returns
    ///
    /// Returns an `Option<EventStaff>` representing the contact event
    /// for the given command contact. If the staff object is present, the contacts field will
    pub fn get_contact_event(&self, cmd_contact: Box<Option<CmdContact>>) -> Option<EventStaff> {
        let staff = self.compose_staff(Box::new(None), true);

        let contacts = self.compose_contact(cmd_contact);
        if let Some(mut staff_opt) = staff {
            staff_opt.contacts = contacts;

            return Some(staff_opt);
        }

        return staff;
    }

    /// Returns the staff information, including address and contact details,
    /// for a given staff member.
    ///
    /// # Arguments
    ///
    /// * `staff` - The staff member for whom to retrieve information.
    /// * `staff_only` - Specifies whether to include address and contact details only
    ///                  if the staff member is currently active and in contract.
    ///
    /// # Returns
    ///
    /// Returns an optional `EventStaff` struct containing the staff information,
    /// or `None` if the staff member does not exist.
    pub(crate) fn get_staff(&self, staff: &dyn StaffIn, staff_only: bool) -> Option<EventStaff> {
        let address = |staff_only: bool| -> Vec<EventAddress>{
            if staff_only {
                return self.compose_address(Box::new(None));
            }

            return Vec::<EventAddress>::new();
        };

        let contact = |staff_only: bool| -> Vec<EventContact>{
            if staff_only {
                return self.compose_contact(Box::new(None));
            }

            return Vec::<EventContact>::new();
        };


        let staff_upd: EventStaff = EventStaff {
            id: staff.id().to_string(),
            first_name: staff.first_name().to_string(),
            last_name: staff.last_name().to_string(),
            vehicle_reg: staff.vehicle_reg().to_string(),
            driver_license: staff.driver_license().to_string(),
            in_contract: staff.in_contract(),
            active: staff.active(),
            address: address(staff_only),
            contacts: contact(staff_only),
            operation:Operation::On
        };

        return Some(staff_upd);
    }


    /// Composes the event staff based on the given option staff and staff_only flag.
    ///
    /// # Arguments
    ///
    /// * `opt_staff` - An optional staff object in a boxed option.
    /// * `staff_only` - A boolean flag indicating whether to include only staff members or all attendees.
    ///
    /// # Returns
    ///
    /// An optional event staff object.
    ///
    pub(crate) fn compose_staff(&self, opt_staff: Box<Option<CmdStaff>>, staff_only: bool) -> Option<EventStaff> {
        let optional_staff = *opt_staff;

        let stf = return match optional_staff {
            Some(staff) => {
                let result_staff = self.get_staff(&staff, staff_only);

                result_staff
            }
            None => {
                let result_staff = self.get_staff(self, staff_only);

                result_staff
            }
        };
    }


    /// Returns an `EventAddress` based on the given `AddressIn`.
    ///
    /// # Arguments
    ///
    /// * `address` - An implementation of the `AddressIn` trait.
    ///
    /// # Returns
    ///
    /// An `EventAddress` struct containing the address details.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let address = Address::new(...);
    /// let event_address = get_address(&address);
    /// ```
    pub(crate) fn get_address(&self, address: &dyn AddressIn) -> EventAddress {
        let event_address = EventAddress {
            id: address.id().to_string(),
            street: address.street().to_string(),
            state: address.state().to_string(),
            post_code: address.post_code().to_string(),
            country: address.country().to_string(),
            staff_id: address.staff_id().to_string(),
            primary: address.primary(),
            operation:Operation::On
        };

        return event_address;
    }


    /// Composes a list of event addresses based on an optional address and a vector of addresses.
    ///
    /// # Arguments
    ///
    /// * `opt_add` - An optional address wrapped in a `Box<Option<CmdAddress>>`.
    ///
    /// # Returns
    ///
    /// A vector of `EventAddress`es containing the composed addresses.
    pub(crate) fn compose_address(&self, opt_add: Box<Option<CmdAddress>>) -> Vec<EventAddress> {
        let mut address_list: Vec<EventAddress> = Vec::new();

        let optional_address = *opt_add;
        match optional_address {
            Some(address) => {
                let address_composed = self.get_address(&address);
                address_list.push(address_composed);
            }
            None => {}
        };

        for addr in &self.address {
            let address_composed=self.get_address(addr);
            address_list.push(address_composed);
        }

        return address_list;
    }


    /// Creates an `EventContact` object from the given `ContactIn` object.
    ///
    /// # Arguments
    ///
    /// * `contact` - A reference to an object that implements the `ContactIn` trait.
    ///
    /// # Returns
    ///
    /// An `EventContact` object containing the information from the `ContactIn` object.
    pub(crate) fn get_contact(&self, contact: &dyn ContactIn) -> EventContact {
        let event_contact = EventContact {
            id: contact.id().to_string(),
            contact_type_id: contact.contact_type_id().to_string(),
            contact_value: contact.contact_value().to_string(),
            staff_id: contact.staff_id().to_string(),
            primary: contact.primary(),
            operation:Operation::On
        };

        return event_contact;
    }


    /// Composes a list of `EventContact` objects based on the given `CmdContact` and existing contacts.
    ///
    /// # Arguments
    ///
    /// * `opt_con` - An optional `CmdContact` wrapped in a `Box<Option<CmdContact>>` object.
    ///
    /// # Returns
    ///
    /// A vector of `EventContact` objects.
    ///
    /// # Example
    ///
    /// ```
    /// let opt_con = Box::new(Some(cmd_con));
    /// let result = compose_contact(&self, opt_con);
    /// ```
    pub(crate) fn compose_contact(&self, opt_con: Box<Option<CmdContact>>) -> Vec<EventContact> {
        let mut contacts_list: Vec<EventContact> = Vec::new();

        let optional_contact = *opt_con;
        match optional_contact {
            Some(con) => {
               let contact_composed = self.get_contact(&con);
               contacts_list.push(contact_composed);
            }
            None => {}
        }


        for con in &self.contacts {
            let contact_composed=self.get_contact(con);
            contacts_list.push(contact_composed);
        }

        return contacts_list;
    }
}