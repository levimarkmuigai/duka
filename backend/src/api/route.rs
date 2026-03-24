use actix_web::web;

use crate::api::handler::{healthcheck, register_merchant};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/healthcheck", web::get().to(healthcheck))
            .route("/register_merchant", web::post().to(register_merchant)),
    );
}
