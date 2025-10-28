use std::collections::HashMap;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    pub codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        self.codes.remove(email);
        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some((id, code)) => Ok((id.clone(), code.clone())),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_code() {
        let mut store = HashmapTwoFACodeStore::default();
        store
            .add_code(
                Email::parse("foo.bar@gmail.com").unwrap(),
                LoginAttemptId::default(),
                TwoFACode::default(),
            )
            .await
            .unwrap();
        assert!(!store.codes.is_empty());
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut store = HashmapTwoFACodeStore::default();
        store
            .add_code(
                Email::parse("foo.bar@gmail.com").unwrap(),
                LoginAttemptId::default(),
                TwoFACode::default(),
            )
            .await
            .unwrap();
        store
            .remove_code(&Email::parse("foo.bar@gmail.com").unwrap())
            .await
            .unwrap();
        assert!(store.codes.is_empty());
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::parse("foo.bar@gmail.com").unwrap();
        let la_id = LoginAttemptId::default();
        let two_fa = TwoFACode::default();

        store
            .add_code(email.clone(), la_id.clone(), two_fa.clone())
            .await
            .unwrap();

        let (login_attempt_id, two_fa_code) = store
            .get_code(&email)
            .await
            .unwrap();

        assert_eq!(login_attempt_id, la_id);
        assert_eq!(two_fa, two_fa_code);
    }
}
