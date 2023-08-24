use actix_web::web;

use crate::handlers::course::{get_course_detail, get_courses_for_teacher, get_all_course};
use crate::handlers::general::health_check_handler;
use crate::handlers::teacher::{ get_all_teachers, get_teacher_details};

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/courses")
            .route("", web::get().to(get_all_course))
            // .route("/", web::post().to(post_new_course))
            .route("/{user_id}", web::get().to(get_courses_for_teacher))
            .route("/{user_id}/{course_id}", web::get().to(get_course_detail)),
    );
}

pub fn teacher_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/teacher")
            .route("", web::get().to(get_all_teachers))
            // .route("", web::post().to(post_new_teacher))
            .route("/{teacher_id}", web::get().to(get_teacher_details))
            // .route("/{teacher_id}", web::delete().to(delete_teacher)),
    );
}
