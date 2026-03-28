use std::collections::HashMap;

use actix_session::storage::{LoadError, SaveError, SessionKey, SessionStore, UpdateError};
use anyhow::{self, Ok};
use futures_util::FutureExt;
use rand::RngExt;
use sqlx::{PgPool, types::time::OffsetDateTime};

type SessionState = HashMap<String, String>;
#[derive(Clone, Debug)]
pub struct PgPoolSession {
    pool: sqlx::PgPool,
}

impl PgPoolSession {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn generate_secure_id() -> String {
        rand::rng()
            .sample_iter(&rand::distr::Alphanumeric)
            .take(64)
            .map(char::from)
            .collect()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PgSessionError {
    #[error("Database failure: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Data serialization failure: {0}")]
    Serde(#[from] serde_json::Error),
}

impl SessionStore for PgPoolSession {
    fn save(
        &self,
        session_state: SessionState,
        ttl: &actix_web::cookie::time::Duration,
    ) -> impl Future<Output = Result<SessionKey, actix_session::storage::SaveError>> {
        let pool = self.pool.clone();

        let ttl_duration = *ttl;

        async move {
            let generated_id_string = Self::generate_secure_id();
            let session_key: SessionKey = generated_id_string
                .clone()
                .try_into()
                .map_err(|e| SaveError::Other(anyhow::anyhow!("Key Creation failed: {}", e)))?;

            let id_str = session_key.as_ref().to_string();

            let json_data = serde_json::to_value(&session_state)
                .map_err(|e| SaveError::Serialization(anyhow::anyhow!(PgSessionError::from(e))))?;

            let expires_at = OffsetDateTime::now_utc() + ttl_duration;

            sqlx::query!(
                "INSERT INTO sessions(id, data, expires_at) VALUES($1,$2,$3)",
                id_str,
                json_data,
                expires_at
            )
            .execute(&pool)
            .await
            .map_err(|e| SaveError::Other(anyhow::anyhow!(PgSessionError::from(e))))?;

            std::result::Result::Ok(session_key)
        }
        .boxed()
    }

    fn load(
        &self,
        session_key: &SessionKey,
    ) -> impl Future<Output = Result<Option<SessionState>, LoadError>> {
        let pool = self.pool.clone();
        let id_str = session_key.as_ref().to_string();

        async move {
            let record = sqlx::query!(
                r#"
                SELECT data FROM sessions
                WHERE id=$1 AND expires_at > NOW()"#,
                id_str
            )
            .fetch_optional(&pool)
            .await
            .map_err(|e| LoadError::Other(PgSessionError::from(e).into()))?;

            match record {
                Some(row) => {
                    let state: SessionState = serde_json::from_value(row.data).map_err(|e| {
                        LoadError::Deserialization(anyhow::anyhow!(PgSessionError::from(e)))
                    })?;

                    std::result::Result::Ok(Some(state))
                }
                None => std::result::Result::Ok(None),
            }
        }
    }

    fn update(
        &self,
        session_key: SessionKey,
        session_state: SessionState,
        ttl: &actix_web::cookie::time::Duration,
    ) -> impl Future<Output = Result<SessionKey, actix_session::storage::UpdateError>> {
        let pool = self.pool.clone();
        let id_str = session_key.as_ref().to_string();
        let ttl_duration = *ttl;

        async move {
            let json_data = serde_json::to_value(&session_state).map_err(|e| {
                UpdateError::Serialization(anyhow::anyhow!(PgSessionError::from(e)))
            })?;

            let expires_at = OffsetDateTime::now_utc() + ttl_duration;

            sqlx::query!(
                r#"
                INSERT INTO sessions (id,data,expires_at)
                VALUES ($1,$2,$3)
                ON CONFLICT (id)
                DO UPDATE SET data = EXCLUDED.data, expires_at = EXCLUDED.expires_at
                "#,
                id_str,
                json_data,
                expires_at
            )
            .execute(&pool)
            .await
            .map_err(|e| UpdateError::Other(anyhow::anyhow!(PgSessionError::from(e))))?;

            std::result::Result::Ok(session_key)
        }
    }

    fn update_ttl(
        &self,
        session_key: &SessionKey,
        ttl: &actix_web::cookie::time::Duration,
    ) -> impl Future<Output = Result<(), anyhow::Error>> {
        let pool = self.pool.clone();
        let id_str = session_key.as_ref().to_string();
        let ttl_duration = *ttl;

        let expires_at = OffsetDateTime::now_utc() + ttl_duration;

        async move {
            sqlx::query!(
                "UPDATE sessions SET expires_at = $1 WHERE id = $2",
                expires_at,
                id_str
            )
            .execute(&pool)
            .await
            .map_err(|e| anyhow::anyhow!(PgSessionError::from(e)))?;

            Ok(())
        }
        .boxed()
    }

    fn delete(&self, session_key: &SessionKey) -> impl Future<Output = Result<(), anyhow::Error>> {
        let pool = self.pool.clone();
        let id_str = session_key.as_ref().to_string();

        async move {
            sqlx::query!("DELETE FROM sessions WHERE id=$1", id_str)
                .execute(&pool)
                .await
                .map_err(|e| anyhow::anyhow!(PgSessionError::from(e)))?;

            Ok(())
        }
        .boxed()
    }
}
