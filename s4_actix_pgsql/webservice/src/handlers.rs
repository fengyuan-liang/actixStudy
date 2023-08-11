use actix_web::{HttpResponse, web};

use crate::db_access::{get_course_details_db, get_courses_for_teacher_db, post_new_course_db};

use super::models::Course;
use super::result::R;
use super::state::AppState;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    println!("incoming for health check");
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new course");
    let course = post_new_course_db(
        &app_state.db,
        new_course.into()
    ).await;
    HttpResponse::Ok()
        .json(
            R::ok(
                course
            )
        )
}

// 请求参数获取，假设路径为`xxx:80/{teacher_id}`
pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, )>,
) -> HttpResponse {
    // 从元组中获取参数
    let (teacher_id, ) = params.into_inner();
    let teacher_id = i32::try_from(teacher_id).unwrap();
    let courses = get_courses_for_teacher_db(&app_state.db, teacher_id).await;;
    HttpResponse::Ok().json(R::ok(courses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let (teacher_id, course_id) =  params.into_inner();
    let teacher_id = i32::try_from(teacher_id).unwrap();
    let course_id = i32::try_from(course_id).unwrap();
    let course = get_course_details_db(&app_state.db, teacher_id, course_id).await;;
    HttpResponse::Ok().json(R::ok(course))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    // use chrono::NaiveDateTime;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;



    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let course = web::Json(Course {
            teacher_id: 1,
            name: "Test course".into(),
            id: Some(3),
            time: None,
        });

        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let teacher_id: web::Path<(usize, )> = web::Path::from((1, ));
        let resp = get_courses_for_teacher(app_state, teacher_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(usize, usize)> = web::Path::from((1, 1));
        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
