use std::sync::Arc;

use redis::{Commands, Connection, RedisResult};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
use crate::domain::email::Email;

pub struct RedisTwoFACodeStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisTwoFACodeStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl TwoFACodeStore for RedisTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        // TODO:
        // 1. Create a new key using the get_key helper function.
        // 2. Create a TwoFATuple instance.
        // 3. Use serde_json::to_string to serialize the TwoFATuple instance into a JSON string.
        // Return TwoFACodeStoreError::UnexpectedError if serialization fails.
        // 4. Call the set_ex command on the Redis connection to set a new key/value pair with an expiration time (TTL).
        // The value should be the serialized 2FA tuple.
        // The expiration time should be set to TEN_MINUTES_IN_SECONDS.
        // Return TwoFACodeStoreError::UnexpectedError if casting fails or the call to set_ex fails.

        let key = get_key(&email);
        let two_fa_code = serde_json::to_string(&TwoFATuple(
            String::from(login_attempt_id.as_ref()),
            String::from(code.as_ref()),
        ))
        .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;

        let mut redis_connection = self.conn.write().await;
        let result: RedisResult<()> =
            redis_connection.set_ex(key, two_fa_code, TEN_MINUTES_IN_SECONDS);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(TwoFACodeStoreError::UnexpectedError),
        }
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        // TODO:
        // 1. Create a new key using the get_key helper function.
        // 2. Call the del command on the Redis connection to delete the 2FA code entry.
        // Return TwoFACodeStoreError::UnexpectedError if the operation fails.

        let key = get_key(email);

        let mut redis_connection = self.conn.write().await;
        let result: RedisResult<()> = redis_connection.del(key);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(TwoFACodeStoreError::UnexpectedError),
        }
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        // TODO:
        // 1. Create a new key using the get_key helper function.
        // 2. Call the get command on the Redis connection to get the value stored for the key.
        // Return TwoFACodeStoreError::LoginAttemptIdNotFound if the operation fails.
        // If the operation succeeds, call serde_json::from_str to parse the JSON string into a TwoFATuple.
        // Then, parse the login attempt ID string and 2FA code string into a LoginAttemptId and TwoFACode type respectively.
        // Return TwoFACodeStoreError::UnexpectedError if parsing fails.

        let key = get_key(email);

        let mut redis_connection = self.conn.write().await;
        let result: RedisResult<String> = redis_connection.get(key);
        match result {
            Ok(serialized_value) => {
                let deserilize_result = serde_json::from_str::<TwoFATuple>(&serialized_value);
                match deserilize_result {
                    Ok(two_fa_touple) => Ok((
                        LoginAttemptId::parse(two_fa_touple.0).unwrap(),
                        TwoFACode::parse(two_fa_touple.1).unwrap(),
                    )),
                    Err(_) => Err(TwoFACodeStoreError::UnexpectedError),
                }
            }
            Err(_) => Err(TwoFACodeStoreError::UnexpectedError),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);

const TEN_MINUTES_IN_SECONDS: u64 = 600;
const TWO_FA_CODE_PREFIX: &str = "two_fa_code:";

fn get_key(email: &Email) -> String {
    format!("{}{}", TWO_FA_CODE_PREFIX, email.as_ref())
}
