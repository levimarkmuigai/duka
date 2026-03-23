use sqlx::PgPool;

use crate::domain::merchant::Merchant;

pub async fn create_merchent(
    pool: PgPool,
    merchant: Merchant,
) -> Result<(), Box<dyn std::error::Error>> {
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
