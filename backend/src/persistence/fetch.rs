use sqlx::PgPool;
use tracing::instrument;

#[instrument(
    name = "db_find_password_by_email"
    skip(pool)
)]
pub async fn find_password_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<String>, sqlx::Error> {
    let password = sqlx::query_scalar!("SELECT password FROM sellers WHERE email = $1", email)
        .fetch_optional(pool)
        .await?;

    Ok(password)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::merchant::{Id, Password};
    #[sqlx::test]
    async fn test_find_password_returns_password_for_valid_user(pool: PgPool) {
        let id = Id::id();
        let email = "test@gmail.com".to_string();
        let password = Password::hash("strongpassword").unwrap();

        sqlx::query!(
            "INSERT INTO sellers VALUES($1,$2,$3)",
            id,
            email.clone(),
            password.clone()
        )
        .execute(&pool)
        .await
        .expect("FAILED TO INSERT merchant");

        let saved_password = find_password_by_email(&pool, &email).await.unwrap();

        assert_eq!(saved_password, Some(password));
    }

    #[sqlx::test]
    async fn test_find_password_returns_none_for_missing_user(pool: PgPool) {
        let email = "rogue@gmail.com".to_string();

        let result = find_password_by_email(&pool, &email).await.unwrap();

        assert!(
            result.is_none(),
            "Expected None for an unsaved email, but got Some"
        );
    }
}
