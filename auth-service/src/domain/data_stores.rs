use crate::domain::{email::Email, password::Password, user::User};
use rand::{thread_rng, Rng};

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

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}

#[derive(Debug)]
pub enum BannedTokenStoreError {
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait TwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: String) -> Result<Self, String> {
        let parsed = uuid::Uuid::try_parse(&id);
        match parsed {
            Ok(_) => Ok(LoginAttemptId(id)),
            Err(e) => Err(format!("Invalid UUID: {}", e)),
        }
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        let id = uuid::Uuid::new_v4();
        LoginAttemptId(id.to_string())
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        if code.chars().count() != 6 {
            return Err("This is not a valid code".to_string());
        }

        match code.parse::<i32>(){
            Ok(_) => Ok(TwoFACode(code)),
            Err(_) => Err("The code is not a valid number".to_string()),
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let mut rng = thread_rng();
        let n: u32 = rng.gen_range(100000..=999999);

        TwoFACode(format!("{}", n))
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
