use sqlx::PgPool;
use tracing::instrument;

use crate::domain::merchant::Merchant;

#[instrument(
    name = "db_create_merchent"
    skip(pool,merchant)
)]
pub async fn create_merchent(pool: PgPool, merchant: &Merchant) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO sellers (id,email, password)
    VALUES($1,$2,$3)
    "#,
        merchant.id.value(),
        merchant.email,
        merchant.password.value(),
    )
    .execute(&pool)
    .await?;
    Ok(())
}
