use std::ops::Add;
use std::option::Option;
use crate::domain::common;
use crate::domain::common::EntityType;
use std::any::Any;

impl AggStaff {
    pub fn get_staff_event(&self, cmd_staff: Box<Option<CmdStaff>>) -> Option<EventStaff> {
        let staff = self.compose_staff(cmd_staff, true);

        return staff;
    }

    pub fn get_address_event(&self, cmd_address: Box<Option<AggAddress>>) -> Option<EventStaff> {
        let staff = self.compose_staff(Box::new(None), true);

        let address = self.compose_address(cmd_address);

        if let Some(mut staff_opt) = &staff {
            staff_opt.address = address;

            return Some(staff_opt);
        }

        return staff;
    }

    pub fn get_contact_event(&self, cmd_contact: Box<Option<AggContact>>) -> Option<EventStaff> {
        let staff = self.compose_staff(Box::new(None), true);

        let contacts = self.compose_contact(cmd_contact);
        if let Some(mut staff_opt) = &staff {
            staff_opt.contacts = contacts;

            return Some(staff_opt);
        }

        return staff;
    }

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
        };

        return Some(staff_upd);
    }

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

    pub(crate) fn compose_address(&self, opt_add: Box<Option<AggAddress>>) -> Vec<EventAddress> {
        let mut address: Vec<EventAddress> = Vec::new();

        let address_func = |add_vec: &mut Vec<EventAddress>, addr: &AggAddress| {
            add_vec.push(EventAddress {
                id: addr.id.to_string(),
                street: addr.street.to_string(),
                state: addr.state.to_string(),
                post_code: addr.post_code.to_string(),
                country: addr.country.to_string(),
                staff_id: addr.staff_id.to_string(),
                primary: addr.primary,
            })
        };

        for addr in &self.address {
            address_func(&mut address, addr)
        }
        let optional_address = *opt_add;
        match optional_address {
            Some(add) => {
                address_func(&mut address, &add)
            }
            None => {}
        }
        return address;
    }
    pub(crate) fn compose_contact(&self, opt_con: Box<Option<AggContact>>) -> Vec<EventContact> {
        let mut contacts: Vec<EventContact> = Vec::new();
        let contact_func = |con_vec: &mut Vec<EventContact>, con: &AggContact| {
            con_vec.push(EventContact {
                id: con.id.to_string(),
                contact_type_id: con.contact_type_id.to_string(),
                contact_value: con.contact_value.to_string(),
                staff_id: con.staff_id.to_string(),
                primary: con.primary,
            })
        };

        for con in &self.contacts {
            contact_func(&mut contacts, con)
        }
        let optional_contact = *opt_con;
        match optional_contact {
            Some(con) => {
                contact_func(&mut contacts, &con)
            }
            None => {}
        }
        return contacts;
    }
}