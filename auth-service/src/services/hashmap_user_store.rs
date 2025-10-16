use std::collections::HashMap;

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    email::Email,
    password::Password,
    user::User,
};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        };
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: Email) -> Result<User, UserStoreError> {
        match self.users.get(&email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError> {
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
        let user = User::new(
            "foo.bar@gmail.com".to_owned(),
            "thePassword".to_owned(),
            false,
        );
        store.add_user(user).await.unwrap();
        assert!(!store.users.is_empty());
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "foo.bar@gmail.com".to_owned(),
            "thePassword".to_owned(),
            false,
        );
        store.users.insert(user.email.clone(), user.clone());
        let user = store
            .get_user(Email::parse("foo.bar@gmail.com").unwrap())
            .await
            .unwrap();
        assert_eq!(user.email.as_ref(), "foo.bar@gmail.com");
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user = User::new(
            "foo.bar@gmail.com".to_owned(),
            "thePassword".to_owned(),
            false,
        );
        store.users.insert(user.email.clone(), user.clone());
        let error = store
            .validate_user(
                Email::parse("foo.bar@gmail.com").unwrap(),
                Password::parse("notThePassword").unwrap(),
            )
            .await;
        assert_eq!(error, Err(UserStoreError::InvalidCredentials));
    }
}
