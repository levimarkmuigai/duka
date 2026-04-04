use actix_session::Session;
use actix_web::{HttpResponse, web};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlx::PgPool;
use tracing::instrument;

use crate::{
    api::errors::Errors,
    domain::merchant::{AuthPayload, Id, Merchant},
    persistence::{create_merchent, fetch::find_merchant_by_email},
};
pub async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[instrument(
    name = "register_merchant_handler",
    skip(payload, pool),
    fields(
        merchant_email = %payload.email
    )
)]
pub async fn register_user(
    payload: web::Json<AuthPayload>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Errors> {
    let auth_payload = payload.into_inner();

    let id = Id::id();
    let email = auth_payload.email;
    let password = auth_payload.password;

    let merchant = Merchant::new(id, email, &password)?;
    create_merchent::create_merchent(&pool, &merchant).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Registration successful",
        "user_id": merchant.id.value()
    })))
}

#[instrument(
    name = "login_handler"
    skip(payload,sessions, pool)
    fields(
        request_email = %payload.email
    )
)]
pub async fn login(
    payload: web::Json<AuthPayload>,
    pool: web::Data<PgPool>,
    sessions: Session,
) -> Result<HttpResponse, Errors> {
    let auth_payload = payload.into_inner();
    let result = find_merchant_by_email(&pool, &auth_payload.email).await?;

    if let Some(merchant) = result {
        let hash = PasswordHash::new(merchant.password.value())?;

        if Argon2::default()
            .verify_password(auth_payload.password.as_bytes(), &hash)
            .is_ok()
        {
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "redirect": "/admin/dashboard"
            })));
        }

        sessions.insert("user_id", merchant.id.value())?;
    };

    Ok(HttpResponse::Unauthorized().json(serde_json::json!({
        "status": "error",
        "message": "Invalid email or password"
    })))
}
