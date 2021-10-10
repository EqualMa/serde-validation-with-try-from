//! Valiate fields in serde with `TryFrom`

use serde::Deserialize;
use std::convert::TryFrom;

fn validate_email(email: &str) -> bool {
    email.contains("@")
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(try_from = "String")]
pub struct Email(String);

impl Email {
    // Here we use a String to represent error just for simplicity
    // You can define a custom enum type like EmailParseError in your application
    pub fn try_new(email: String) -> Result<Self, String> {
        if validate_email(&email) {
            Ok(Self(email))
        } else {
            Err(format!("Invalid email {}", email))
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn inner(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for Email {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct User {
    name: String,
    email: Email,
}

#[cfg(test)]
mod tests {
    use super::{Email, User};

    #[test]
    fn invalid_email() {
        let res: Result<Email, _> = serde_json::from_str("\"invalid_email\"");
        let err = res.unwrap_err();
        assert_eq!(format!("{}", err), "Invalid email invalid_email");
    }

    #[test]
    fn valid_email() {
        let email: Email = serde_json::from_str("\"valid_email@example.com\"").unwrap();
        assert_eq!(email, Email("valid_email@example.com".to_string()));
        assert_eq!(email.inner(), "valid_email@example.com");
    }

    #[test]
    fn invalid_user() {
        let res: Result<User, _> =
            serde_json::from_str(r#"{"name": "Alice", "email": "invalid_email"}"#);
        let err = res.unwrap_err();
        assert_eq!(
            format!("{}", err),
            format!(
                "Invalid email invalid_email at line {} column {}",
                err.line(),
                err.column(),
            ),
        );
    }

    #[test]
    fn valid_user() {
        let user: User =
            serde_json::from_str(r#"{"name": "Alice", "email": "valid_email@example.com"}"#)
                .unwrap();

        assert_eq!(
            user,
            User {
                name: "Alice".to_string(),
                email: Email("valid_email@example.com".to_string()),
            },
        );
    }
}
