use actix_web::web;

use crate::handlers::course::{get_course_detail, get_courses_for_teacher, post_new_course};
use crate::handlers::general::health_check_handler;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("/", web::post().to(post_new_course))
            .route("/{user_id}", web::get().to(get_courses_for_teacher))
            .route("/{user_id}/{course_id}", web::get().to(get_course_detail)),
    );
}
