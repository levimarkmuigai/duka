use sqlx::PgPool;
use tracing::instrument;

use crate::domain::merchant::Merchant;

#[instrument(
    name = "db_find_merchant_by_email"
    skip(pool)
)]
pub async fn find_merchant_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<Merchant>, sqlx::Error> {
    let merchant = sqlx::query_as::<_, Merchant>("SELECT * FROM sellers WHERE email=$1")
        .bind(email)
        .fetch_optional(pool)
        .await?;

    Ok(merchant)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::merchant::{Id, Password};
    #[sqlx::test]
    async fn test_find_merchant_returns_merchant_for_valid_user(pool: PgPool) {
        let id = Id::id();
        let email = "test@gmail.com".to_string();
        let password = Password::hash("strongpassword").unwrap();

        sqlx::query!("INSERT INTO sellers VALUES($1,$2,$3)", id, email, password)
            .execute(&pool)
            .await
            .expect("FAILED TO INSERT merchant");

        let saved_merchant = find_merchant_by_email(&pool, &email).await.unwrap();

        if let Some(merchant) = saved_merchant {
            assert_eq!(merchant.id.value(), &id);
            assert_eq!(merchant.password.value(), &password);
        }
    }

    #[sqlx::test]
    async fn test_find_merchant_returns_none_for_missing_user(pool: PgPool) {
        let email = "rogue@gmail.com".to_string();

        let result = find_merchant_by_email(&pool, &email).await.unwrap();

        assert!(
            result.is_none(),
            "Expected None for an unsaved email, but got Some"
        );
    }
}
