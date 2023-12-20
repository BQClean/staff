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

            let ops = || -> Operation {
                match add.operation {
                    Operation::On =>  Operation::On,
                    Operation::Off => Operation::Off,
                }
            };

            let address=AggAddress{
                id: add.id.to_string(),
                street: add.street.to_string(),
                state: add.state.to_string(),
                post_code: add.post_code.to_string(),
                country: add.country.to_string(),
                staff_id: add.staff_id.to_string(),
                primary: add.primary,
                operation:ops()
            };

            self.address.push(address)
        }
    }

    pub(crate) fn process_address_updated_event(&mut self, event: &CommonEvent<EventStaff>) {}

    pub(crate) fn process_contact_created_event(&mut self, event: &CommonEvent<EventStaff>) {}

    pub(crate) fn process_contact_updated_event(&mut self, event: &CommonEvent<EventStaff>) {}
}