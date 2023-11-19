
pub enum  Commands {
    CreateStaff(CreateStaffCommand),
    UpdateStaff(UpdateStaffCommand),
    InactiveStaff(InactivateStaffCommand),
    CreateAddress(CreateAddressCommand),
    UpdateAddress(UpdateAddressCommand)
}