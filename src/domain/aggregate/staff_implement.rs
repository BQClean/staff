use std::ops::Add;
use std::option::Option;

impl RootStaff {
    fn compose_staff(&self, opt_staff: Box<Option<Staff>>) -> Box<Option<EventStaff>> {
        let optional_staff = *opt_staff;
        return match optional_staff {
            Some(staff) => {
                let staff_upd = EventStaff {
                    id: staff.id.to_string(),
                    first_name: staff.first_name.to_string(),
                    last_name: staff.last_name.to_string(),
                    vehicle_reg: staff.vehicle_reg.to_string(),
                    driver_license: staff.driver_license.to_string(),
                    in_contract: staff.in_contract,
                    active: false,
                    address: self.compose_address(Box::new(None)),
                    contacts: self.compose_contact(Box::new(None)),
                };
                Box::new(Some(staff_upd))
            }
            None => {
                Box::new(None)
            }
        };
    }
    fn compose_address(&self, opt_add: Box<Option<Address>>) -> Vec<EventAddress> {
        let mut address: Vec<EventAddress> = Vec::new();
        let address_func = |add_vec: &mut Vec<EventAddress>, addr: &Address| {
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

    fn compose_contact(&self, opt_con: Box<Option<Contact>>) -> Vec<EventContact> {
        let mut contacts: Vec<EventContact> = Vec::new();
        let contact_func = |con_vec: &mut Vec<EventContact>, con: &Contact| {
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