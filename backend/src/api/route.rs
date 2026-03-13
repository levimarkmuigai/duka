use actix_web::web;

use crate::api::handler::healthcheck;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").route("/healthcheck", web::get().to(healthcheck)));
}
