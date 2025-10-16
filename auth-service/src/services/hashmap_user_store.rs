use std::collections::HashMap;

use crate::domain::{data_stores::{UserStore, UserStoreError}, user::User};


#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(user.email.as_str()) {
            return Err(UserStoreError::UserAlreadyExists);
        };
        self.users.insert(user.email.to_owned(), user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.get_user(email).await?;
        if user.password == password {
            Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}

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
            }).await
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
        let user = store.get_user("foo.bar@gmail.com").await.unwrap();
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
        let error = store.validate_user("foo.bar@gmail.com", "notThePassword").await;
        assert_eq!(error, Err(UserStoreError::InvalidCredentials));
    }
}
