use actix_web::{HttpResponse, http::header::LOCATION, web};
use argon2::{Argon2, PasswordHash, PasswordVerifier};

use crate::{
    api::errors::Errors,
    domain::merchant::{AuthPayload, Id, Merchant},
    persistence::{create_merchent, db::get_pool, fetch::find_password_by_email},
};
pub async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn register_merchant(payload: web::Json<AuthPayload>) -> Result<HttpResponse, Errors> {
    let auth_payload = payload.into_inner();

    let id = Id::id();
    let email = auth_payload.email;
    let password = auth_payload.password;

    let merchant = Merchant::new(id, email, password)?;

    let pool = get_pool().await?;
    create_merchent::create_merchent(pool, &merchant).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Registration successful",
        "user_id": merchant.id.value()
    })))
}

pub async fn login(payload: web::Json<AuthPayload>) -> Result<HttpResponse, Errors> {
    let auth_payload = payload.into_inner();

    let pool = get_pool().await?;
    let password = find_password_by_email(&pool, &auth_payload.email).await?;

    if let Some(db_password) = password {
        let hash = PasswordHash::new(&db_password)?;

        if Argon2::default()
            .verify_password(auth_payload.password.as_bytes(), &hash)
            .is_ok()
        {
            return Ok(HttpResponse::SeeOther()
                .insert_header((LOCATION, "admin/dashboard"))
                .finish());
        }
    };

    Ok(HttpResponse::Unauthorized().json(serde_json::json!({
        "status": "error",
        "message": "Invalid email or password"
    })))
}
