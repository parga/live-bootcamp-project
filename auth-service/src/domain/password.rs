#[derive(Debug)]
pub enum PasswordError {
    InvalidPassword,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password{
    pub fn parse(password: &str) -> Result<Self, PasswordError> {
        if password.len() < 8 {
            return Err(PasswordError::InvalidPassword);
        }
        Ok(Self(password.to_string()))
    }
}

impl AsRef<str> for Password{
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}


impl From<String> for Password {
    fn from(value: String) -> Self {
        Self(value)
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_parse_valid() {
        assert!(Password::parse("password1234").is_ok());
    }

    #[test]
    fn emal_parse_invalid() {
        let invalid_passwords = vec![
            "",                     // empty
            "5454",                 // too short
        ];

        for password in invalid_passwords {
            assert!(Password::parse(password).is_err(), "Should fail for: {}", password);
        }
    }
}
