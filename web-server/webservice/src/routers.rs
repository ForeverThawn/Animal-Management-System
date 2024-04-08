use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn animal_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(web::scope("/animals") // go 路由组
    .route("/new", web::post().to(post_new_animal))
    // .route("/{id}", web::get().to(get_animals))
    // .route("/login", web::post().to(login))
    .route("/", web::get().to(get_animals))
    .route("/{id}", web::get().to(get_animal_by_id))

    );
}
