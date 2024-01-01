use std::option::Option;
use comentities::staff::commands::{CmdStaffAddress, CmdStaffContact,CmdRootStaff};


//
impl AggStaff {

    pub fn get_staff_event(&self, cmd_staff: Box<Option<&CmdRootStaff>>) -> Option<EventRootStaff> {
        let staff = self.compose_staff(cmd_staff, true);

        return staff;
    }

    pub fn get_address_event(&self, cmd_address: Box<Option<&CmdStaffAddress>>) -> Option<EventRootStaff> {
        let staff = self.compose_staff(Box::new(None), true);

        let address = self.compose_address(cmd_address);

        if let Some(mut staff_opt) = staff {
            staff_opt.address = address;

            return Some(staff_opt);
        }

        return staff;
    }

    pub fn get_contact_event(&self, cmd_contact: Box<Option<&CmdStaffContact>>) -> Option<EventRootStaff> {
        let staff = self.compose_staff(Box::new(None), true);

        let contacts = self.compose_contact(cmd_contact);
        if let Some(mut staff_opt) = staff {
            staff_opt.contacts = contacts;

            return Some(staff_opt);
        }

        return staff;
    }

    pub(crate) fn get_staff(&self, staff: &dyn StaffIn, staff_only: bool) -> Option<EventRootStaff> {
        let address = |staff_only: bool| -> Vec<EventStaffAddress>{
            if staff_only {
                return self.compose_address(Box::new(None));
            }

            return Vec::<EventStaffAddress>::new();
        };

        let contact = |staff_only: bool| -> Vec<EventStaffContact>{
            if staff_only {
                return self.compose_contact(Box::new(None));
            }

            return Vec::<EventStaffContact>::new();
        };


        let staff_upd: EventRootStaff = EventRootStaff {
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

    pub(crate) fn compose_staff(&self, opt_staff: Box<Option<&CmdRootStaff>>, staff_only: bool) -> Option<EventRootStaff> {
        let optional_staff = *opt_staff;

        return match optional_staff {
            Some(staff) => {
                let result_staff = self.get_staff(staff, staff_only);

                result_staff
            }
            None => {
                let result_staff = self.get_staff(self, staff_only);

                result_staff
            }
        };
    }

    pub(crate) fn get_address(&self, address: &dyn AddressIn) -> EventStaffAddress {
        let event_address = EventStaffAddress {
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

    pub(crate) fn compose_address(&self, opt_add: Box<Option<&CmdStaffAddress>>) -> Vec<EventStaffAddress> {
        let mut address_list: Vec<EventStaffAddress> = Vec::new();

        let optional_address = *opt_add;
        match optional_address {
            Some(address) => {
                let address_composed = self.get_address(address);
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

    pub(crate) fn get_contact(&self, contact: &dyn ContactIn) -> EventStaffContact {
        let event_contact = EventStaffContact {
            id: contact.id().to_string(),
            contact_type_id: contact.contact_type_id().to_string(),
            contact_value: contact.contact_value().to_string(),
            staff_id: contact.staff_id().to_string(),
            primary: contact.primary(),
            operation:Operation::On
        };

        return event_contact;
    }

    pub(crate) fn compose_contact(&self, opt_con: Box<Option<&CmdStaffContact>>) -> Vec<EventStaffContact> {
        let mut contacts_list: Vec<EventStaffContact> = Vec::new();

        let optional_contact = *opt_con;
        match optional_contact {
            Some(con) => {
               let contact_composed = self.get_contact(con);
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