use actix_web::web;

use crate::{
    api::handler::{healthcheck, login, register_user},
    view::handler::dashboard_view,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/healthcheck", web::get().to(healthcheck))
            .route("/register_user", web::post().to(register_user))
            .route("/login", web::post().to(login)),
    );

    cfg.service(web::scope("/admin").route("dashboard", web::get().to(dashboard_view)));
}
