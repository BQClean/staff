impl AggStaff {
    pub(crate) fn process_staff_created_event(
        &mut self,
        event: &CommonEvent<EventStaff>) {
        self.id = event.data.id.to_string();
        self.driver_license = event.data.driver_license.to_string();
        self.last_name = event.data.last_name.to_string();
        self.first_name = event.data.first_name.to_string();
        self.operation = event.data.operation.clone();
        self.active = event.data.active;
        self.in_contract = event.data.active;
    }

    pub(crate) fn process_staff_updated_event(
        &mut self,
        event: &CommonEvent<EventStaff>) {
        self.id = event.data.id.to_string();
        self.driver_license = event.data.driver_license.to_string();
        self.last_name = event.data.last_name.to_string();
        self.first_name = event.data.first_name.to_string();
        self.operation = event.data.operation.clone();
        self.active = event.data.active;
        self.in_contract = event.data.active;
    }

    pub(crate) fn process_address_created_event(
        &mut self,
        event: &CommonEvent<EventStaff>) {
        for add in event.data.address.iter() {
            let ops = |ad: &EventAddress| -> Operation {
                match ad.operation {
                    Operation::On => Operation::On,
                    Operation::Off => Operation::Off,
                }
            };

            let address = AggAddress {
                id: add.id.to_string(),
                street: add.street.to_string(),
                state: add.state.to_string(),
                post_code: add.post_code.to_string(),
                country: add.country.to_string(),
                staff_id: add.staff_id.to_string(),
                primary: add.primary,
                operation: ops(add),
            };

            self.address.push(address)
        }
    }

    pub(crate) fn process_address_updated_event(
        &mut self,
        event: &CommonEvent<EventStaff>) {
        let ops = |ad: &EventAddress| -> Operation {
            match ad.operation {
                Operation::On => Operation::On,
                Operation::Off => Operation::Off,
            }
        };

        for add in event.data.address.iter() {
            let address = self.address.iter_mut().find(|address: &&mut AggAddress| -> bool{
                address.id == add.id
            });

            match address {
                Some(a) => {
                    a.primary = add.primary;
                    a.country = add.country.to_string();
                    a.staff_id = add.staff_id.to_string();
                    a.state = add.state.to_string();
                    a.post_code = add.post_code.to_string();
                    a.operation = ops(add);
                    a.street = add.street.to_string();
                }
                None => {
                    continue;
                }
            }
        }
    }

    pub(crate) fn process_contact_created_event(
        &mut self,
        event: &CommonEvent<EventStaff>) {
        let ops = |cn: &EventContact| -> Operation {
            match cn.operation {
                Operation::On => Operation::On,
                Operation::Off => Operation::Off,
            }
        };

        for con in event.data.contacts.iter() {
            let contact = AggContact {
                id: con.id.to_string(),
                contact_type_id: con.contact_type_id.to_string(),
                contact_value: con.contact_value.to_string(),
                primary: con.primary,
                staff_id: con.staff_id.to_string(),
                operation: ops(con),
            };

            self.contacts.push(contact);
        }
    }

    pub(crate) fn process_contact_updated_event(
        &mut self,
        event: &CommonEvent<EventStaff>) {
        let ops = |cn: &EventContact| -> Operation {
            match cn.operation {
                Operation::On => Operation::On,
                Operation::Off => Operation::Off,
            }
        };
        for con in event.data.contacts.iter() {
            let contact = self.contacts.iter_mut().find(|co: &&mut AggContact| -> bool{
                co.id == con.id
            });

            match contact {
                Some(cn) => {
                    cn.primary=con.primary;
                    cn.contact_value=con.contact_value.to_string();
                    cn.contact_type_id=con.contact_type_id.to_string();
                    cn.staff_id=con.staff_id.to_string();
                    cn.operation=ops(con);
                }
            }
        }
    }
}