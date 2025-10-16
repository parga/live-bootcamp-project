use crate::domain::{email::Email, password::Password};

#[derive(Debug, Clone)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        let email = Email::parse(&email).unwrap();
        let password = Password::parse(&password).unwrap();
        Self { email, password, requires_2fa }
    }
}
