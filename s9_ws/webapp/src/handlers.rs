use actix_web::{Error, HttpResponse, web};
use log::{error, info};
use serde_json::json;

use crate::webapp::error::MyError;
use crate::webapp::models::{TeacherRegisterForm, TeacherResponse};
use crate::webapp::result::R;

// 我这里返回的是这样的，跟老师不一样
// {"code":200,"data":[{"id":1,"name":"name test","picture_url":"http://xxx.png","profile":"profile_test1"}]}
pub async fn get_all_teacher(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    // 相当于http客户端，可以进行http访问
    let awc_client = awc::Client::default();
    let mut res = awc_client
        .get("http://localhost:3000/teacher")
        .send()
        .await
        .map_err(|e| {
            error!("Failed to send HTTP request: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to send HTTP request")
        })?;

    let body = actix_web::web::Bytes::from(res.body().await.map_err(|e| {
        error!("Failed to read response body: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to read response body")
    })?);

    let body_str = String::from_utf8_lossy(&body);
    // 如果失败 打印响应的结果
    // info!("Response body: {}", body_str);

    let res:R<Vec<TeacherResponse>>= match serde_json::from_str(&body_str) {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to deserialize JSON: {}", e);
            return Err(actix_web::error::ErrorInternalServerError("Failed to deserialize JSON"));
        }
    };

    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("teachers", &res.data);
    // 开始渲染模板
    let s = tmpl
        .render("teachers.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn show_register_form(tmpl:web::Data<tera::Tera>) ->  Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_name", "");
    ctx.insert("current_image_url", "");
    ctx.insert("current_profile", "");

    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl:web::Data<tera::Tera>,
    params: web::Form<TeacherRegisterForm>
) -> Result<HttpResponse, Error>  {
    let mut ctx = tera::Context::new();
    let  s;
    if params.name == "Dave" {
        error!("params.name == Dave");
        ctx.insert("error", "Dave already exists!");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_image_url", &params.image_url);
        ctx.insert("current_profile", &params.profile);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| MyError::TeraError("Template error".to_string()))
            .unwrap()
    } else {
        let new_teacher = json!({
            "name": &params.name,
            "picture_url": &params.image_url,
            "profile": &params.profile
        });
        let awc_client = awc::Client::default();
        let res = awc_client
            .post("http://localhost:3000/teacher")
            .send_json(&new_teacher)
            .await
            .unwrap()
            .body()
            .await?;
        let teacher_response: R<TeacherResponse> = serde_json::from_str(&std::str::from_utf8(&res)?)?;
        info!("teacher_response:{:?}", teacher_response);
        s = format!("congratulations! Your id is: {}", teacher_response.data.id);
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}