use serde::Deserialize;
use std::convert::TryFrom;

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(try_from = "String")]
struct Email(String);

impl Email {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for Email {
    // Here we use a String to represent error just for simplicity
    // You can define a custom enum type like EmailParseError in your application
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.contains("@") {
            Ok(Self(value))
        } else {
            Err("Invalid email".to_string())
        }
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
struct User {
    name: String,
    email: Email,
}

fn main() {
    let res: Result<Email, _> = serde_json::from_str("\"invalid_email\"");
    let err = res.unwrap_err();
    assert_eq!(format!("{}", err), "Invalid email");

    let email: Email = serde_json::from_str("\"valid_email@example.com\"").unwrap();
    assert_eq!(email, Email("valid_email@example.com".to_string()));
    assert_eq!(email.into_inner(), "valid_email@example.com");

    let res: Result<User, _> =
        serde_json::from_str(r#"{"name": "Alice", "email": "invalid_email"}"#);
    let err = res.unwrap_err();
    assert_eq!(
        format!("{}", err),
        format!(
            "Invalid email at line {} column {}",
            err.line(),
            err.column(),
        ),
    );

    assert_eq!(
        User {
            name: "Alice".to_string(),
            email: Email("valid_email@example.com".to_string()),
        },
        serde_json::from_str(r#"{"name": "Alice", "email": "valid_email@example.com"}"#).unwrap(),
    );
}
