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
