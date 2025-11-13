use color_eyre::eyre::{eyre, Result};
use validator::validate_email;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(s: &str) -> Result<Email> {
        if !validate_email(s) {
            return Err(eyre!(format!("{} is not a valid email.", s)));
        }
        Ok(Self(s.to_owned()))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for Email {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn email_parse_valid() {
        let email: String = SafeEmail().fake();
        let result = Email::parse(email.as_str());
        assert!(result.is_ok());
    }

    #[test]
    fn email_parse_invalid() {
        let invalid_emails = vec![
            "",                   // empty
            "plainaddress",       // missing @
            "@no-local-part.com", // missing local part
            "user@.com",          // domain starts with dot
            // "user@com",           // missing dot in domain
            "user@com.",          // domain ends with dot
            "user@-com.com",      // domain starts with dash
            "user@com..com",      // double dot in domain
            "user@.com.",         // domain starts/ends with dot
            "user@com,com",       // comma instead of dot
        ];

        for email in invalid_emails {
            let result = Email::parse(email);
            assert!(result.is_err(), "Should fail for: {}", email);
        }
    }
}
