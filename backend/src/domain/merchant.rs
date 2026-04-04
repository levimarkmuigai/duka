use sqlx::prelude::FromRow;
use thiserror::Error;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow, PartialEq, sqlx::Type)]
#[sqlx(transparent)]
pub struct Id(Uuid);

impl Id {
    pub fn id() -> Uuid {
        Uuid::new_v4()
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl From<Uuid> for Id {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Deserialize, FromRow, PartialEq, sqlx::Type)]
#[sqlx(transparent)]
pub struct Password(String);

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Failed to convert password to struct")]
    Conversion,

    #[error("Failed to hash password")]
    Hashing(#[from] argon2::password_hash::Error),
}

impl Password {
    pub fn hash(p: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let hashed_password = argon2.hash_password(p.as_bytes(), &salt)?.to_string();

        Ok(hashed_password)
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

impl TryFrom<String> for Password {
    type Error = PasswordError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Deserialize, FromRow, PartialEq)]
pub struct Merchant {
    pub id: Id,
    pub email: String,
    pub password: Password,
}

impl Merchant {
    pub fn new(id: Uuid, email: String, password: &str) -> Result<Self, PasswordError> {
        let hashed_password = Password::hash(password)?;

        Ok(Self {
            id: Id::from(id),
            email,
            password: Password::try_from(hashed_password)?,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}
