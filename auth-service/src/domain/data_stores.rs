use crate::domain::{email::Email, password::Password, user::User};

#[async_trait::async_trait]
pub trait UserStore: Send + Sync {
    async fn get_user(&self, email: Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError>;
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}
