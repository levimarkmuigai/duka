use sqlx::PgPool;
use tracing::instrument;

use crate::domain::merchant::Merchant;

#[instrument(
    name = "db_create_merchent"
    skip(pool,merchant)
)]
pub async fn create_merchent(pool: &PgPool, merchant: &Merchant) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO sellers (id,email, password)
    VALUES($1,$2,$3)
    "#,
        merchant.id.value(),
        merchant.email,
        merchant.password.value(),
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::merchant::{Id, Merchant, Password};

    #[sqlx::test]
    async fn test_create_merchant_insert(pool: PgPool) {
        let id = Id::id();
        let email = "test_merchant@gmail.com".to_string();
        let password = Password::hash("secure_pass_123").unwrap();

        let merchant = Merchant::new(id, email.clone(), &password).unwrap();

        let result = create_merchent(&pool, &merchant).await;

        assert!(result.is_ok(), "Failed to insert merchant to database");

        let saved_record = sqlx::query!("SELECT email FROM sellers WHERE email = $1", email)
            .fetch_one(&pool)
            .await
            .expect("Merchant not found in the database");

        assert_eq!(saved_record.email, email);
    }
}
