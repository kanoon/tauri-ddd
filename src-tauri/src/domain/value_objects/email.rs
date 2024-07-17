#[derive(Debug, PartialEq, Eq)]
struct Email(String);

impl Email {
    fn new(email: &str) -> Result<Self, String> {
        if Self::is_valid(email) {
            Ok(Self(email.to_string()))
        } else {
            Err(format!("Invalid email: {}", email))
        }
    }

    fn is_valid(email: &str) -> bool {
        email.contains("@") && email.contains(".")
    }
}
