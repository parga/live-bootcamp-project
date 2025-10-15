use std::collections::HashMap;

use crate::domain::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

// TODO: Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.
#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(user.email.as_str()) {
            return Err(UserStoreError::UserAlreadyExists);
        };
        self.users.insert(user.email.to_owned(), user);
        Ok(())
    }

    // TODO: Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    // TODO: Implement a public method called `validate_user`, which takes an
    // immutable reference to self, an email string slice, and a password string slice
    // as arguments. `validate_user` should return a `Result` type containing either a
    // unit type `()` if the email/password passed in match an existing user, or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    // Return `UserStoreError::InvalidCredentials` if the password is incorrect.
    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email)?;
        if user.password == password {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        store
            .add_user(User {
                email: String::from("foo.bar@gmail.com"),
                password: String::from("thePassword"),
                requires_2fa: false,
            })
            .unwrap();
        assert!(!store.users.is_empty());
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        store
            .users
            .insert(
                String::from("foo.bar@gmail.com"),
                User {
                    email: String::from("foo.bar@gmail.com"),
                    password: String::from("thePassword"),
                    requires_2fa: false,
                },
            );
        let user = store.get_user("foo.bar@gmail.com").unwrap();
        assert_eq!(user.email, "foo.bar@gmail.com");
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        store
            .users
            .insert(
                String::from("foo.bar@gmail.com"),
                User {
                    email: String::from("foo.bar@gmail.com"),
                    password: String::from("thePassword"),
                    requires_2fa: false,
                },
            );
        let error = store.validate_user("foo.bar@gmail.com", "notThePassword");
        assert_eq!(error, Err(UserStoreError::InvalidCredentials));
    }
}
