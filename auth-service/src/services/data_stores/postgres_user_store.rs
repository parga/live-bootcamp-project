use std::error::Error;

use argon2::{
    password_hash::{SaltString}, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};

use sqlx::PgPool;

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    email::Email,
    password::Password,
    user::User,
};

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
pub struct UserRow {
    pub email: String,
    pub password_hash: String,
    pub requires_2fa: bool,
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    #[tracing::instrument(name = "Retrieving user from PostgreSQL", skip_all)]
    async fn get_user(&self, email: Email) -> Result<User, UserStoreError> {
        let row = sqlx::query!("SELECT * FROM users WHERE email = $1", email.as_ref())
            .fetch_one(&self.pool)
            .await;

        match row {
            Ok(user_row) => {
                let user = User::new(
                    user_row.email,
                    user_row.password_hash,
                    user_row.requires_2fa,
                );
                Ok(user)
            }
            Err(_) => Err(UserStoreError::UserNotFound),
        }
    }

    #[tracing::instrument(name = "Validating user credentials in PostgreSQL", skip_all)]
    async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError> {
        let row = sqlx::query!("SELECT * FROM users WHERE email = $1", email.as_ref())
            .fetch_one(&self.pool)
            .await;

        match row {
            Ok(user_row) => {
                let verify_password_result =
                    verify_password_hash(user_row.password_hash, password.as_ref().to_owned())
                        .await;
                match verify_password_result {
                    Ok(_) => Ok(()),
                    Err(_) => Err(UserStoreError::InvalidCredentials),
                }
            }
            Err(_) => Err(UserStoreError::UserNotFound),
        }
    }
    #[tracing::instrument(name = "Adding user to PostgreSQL", skip_all)] 
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let password_hash = compute_password_hash(user.password.as_ref().to_owned()).await;
        let password_hash = password_hash.unwrap();

        let existing = sqlx::query!("SELECT * FROM users WHERE email = $1", user.email.as_ref())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| UserStoreError::UnexpectedError(e.into()))?;

        if existing.is_some() {
            return Err(UserStoreError::UserAlreadyExists);
        }

        let row = sqlx::query!(
            "INSERT INTO users (email, password_hash, requires_2fa) VALUES ($1, $2, $3)",
            user.email.as_ref(),
            password_hash,
            user.requires_2fa
        )
        .execute(&self.pool)
        .await;

        match row {
            Ok(_) => Ok(()),
            Err(e) => Err(UserStoreError::UnexpectedError(e.into())),
        }
    }
}

#[tracing::instrument(name = "Verify password hash", skip_all)]
async fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let current_span: tracing::Span = tracing::Span::current();
    let result = tokio::task::spawn_blocking(move || {
        current_span.in_scope(|| {
            let expected_password_hash: PasswordHash<'_> =
                PasswordHash::new(&expected_password_hash)?;

            Argon2::default()
                .verify_password(password_candidate.as_bytes(), &expected_password_hash)
                .map_err(|e| e.into())
        })
    })
    .await;

    result?
}

#[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: String) -> Result<String, Box<dyn Error>> {
    let current_span: tracing::Span = tracing::Span::current();
    let password_hash: Result<String, Box<dyn Error + Send + Sync>> =
        tokio::task::spawn_blocking(move || {
            current_span.in_scope(|| {
                let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
                let  password_hash = Argon2::new(
                    Algorithm::Argon2id,
                    Version::V0x13,
                    Params::new(15000, 2, 1, None)?,
                )
                .hash_password(password.as_bytes(), &salt)?
                .to_string();

                Ok(password_hash)
            })
        })
        .await?;

    match password_hash {
        Ok(password_hash_result) => Ok(password_hash_result),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::compute_password_hash;

    #[tokio::test]
    async fn test_compute_password_hash_returns_hash() {
        let password = "test_password".to_string();
        let hash_result = compute_password_hash(password).await;
        assert!(hash_result.is_ok());
        let hash = hash_result.unwrap();
        // Argon2 hashes start with "$argon2"
        assert!(hash.starts_with("$argon2"));
    }

    #[tokio::test]
    async fn test_verify_password_hash_succeeds() {
        let password = "test_password".to_string();
        let hash = super::compute_password_hash(password.clone())
            .await
            .unwrap();
        let verify_result = super::verify_password_hash(hash, password).await;
        assert!(verify_result.is_ok());
    }

    #[tokio::test]
    async fn test_verify_password_hash_fails() {
        let password = "test_password".to_string();
        let hash = super::compute_password_hash(password.clone())
            .await
            .unwrap();
        let verify_result = super::verify_password_hash(hash, "not_the_password".to_owned()).await;
        assert!(verify_result.is_err());
    }
}
