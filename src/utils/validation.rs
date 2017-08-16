use super::regex::{Regex, Error};

pub fn validate_email(email: String) -> Result<bool, Error> {
    let pattern = "[A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,64}";
    let regex = Regex::new(&pattern);

    regex.map(|re| re.is_match(&email))
}
