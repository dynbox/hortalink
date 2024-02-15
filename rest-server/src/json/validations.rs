use garde::Error;

pub fn validate_name(value: &Option<String>, _: &()) -> garde::Result {
    match value {
        Some(n) => {
            if n.chars().all(char::is_alphabetic) {
                Ok(())
            } else {
                Err(Error::new("Name must contain only letters"))
            }
        }
        None => Ok(()),
    }
}