use actix_web::{HttpResponse, web};
use crate::dbaccess::teacher::{get_all_teachers_db, get_teacher_details_db, post_new_teacher_db};
use crate::error::MyError;
use crate::models::teacher::CreateTeacher;
use crate::result::R;
use crate::state::AppState;

pub async fn get_all_teachers(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    get_all_teachers_db(&app_state.db)
        .await
        .map(|teachers| HttpResponse::Ok().json(R::ok(teachers)))
}

pub async fn get_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_teacher_details_db(&app_state.db, teacher_id)
        .await
        .map(|teacher| HttpResponse::Ok().json(R::ok(teacher)))
}

pub async fn post_new_teacher(
    new_teacher: web::Data<CreateTeacher>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_teacher_db(&app_state.db, CreateTeacher::from(new_teacher))
        .await
        .map(|teacher|  HttpResponse::Ok().json(R::ok(teacher)))
}

// 其他的都一样 不写了