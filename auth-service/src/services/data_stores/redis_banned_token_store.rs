use std::sync::Arc;

use redis::{Commands, Connection, RedisResult};
use tokio::sync::RwLock;

use crate::{
    domain::data_stores::{BannedTokenStore, BannedTokenStoreError},
    utils::auth::TOKEN_TTL_SECONDS,
};

pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        let mut redis_connection = self.conn.write().await;
        let result: RedisResult<()> = redis_connection.set_ex(get_key(token.as_str()), true, TOKEN_TTL_SECONDS as u64);
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(BannedTokenStoreError::UnexpectedError),
        }
    }

    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        let mut redis_connection = self.conn.write().await;
        let result: RedisResult<bool> = redis_connection.exists(get_key(token));
        match result {
            Ok(value) => Ok(value),
            _ => Err(BannedTokenStoreError::UnexpectedError),
        }
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
