

fn validate_unique_staff_id(staff_id: &str) -> Result<(), ValidationError> {
    if staff_id.len() > 0  {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("terrible_username"));
    }

    Ok(())
}