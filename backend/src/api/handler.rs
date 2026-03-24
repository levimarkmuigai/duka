use actix_web::{HttpResponse, web};

use crate::{
    api::errors::Errors,
    domain::merchant::{AuthPayload, Id, Merchant},
    persistence::create_merchent::create_merchent,
    utils,
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

    let pool = utils::get_pool().await?;
    create_merchent(pool, &merchant).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Registration successful",
        "user_id": merchant.id.value()
    })))
}
