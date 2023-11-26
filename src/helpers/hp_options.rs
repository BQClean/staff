
pub fn get_option_value<T>(option_value: Option<T>) -> T {
    match option_value {
        Some(value) => value, // If it's Some, return the value inside
        None => {
            return option_value.unwrap()
        }
    }
}