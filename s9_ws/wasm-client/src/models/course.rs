use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
// use crate::models::course::Course;
use crate::error::MyError;
use std::convert::TryFrom;
use wasm_bindgen::JsCase;
use wasm_bindgen_futures::JsFuture;
use web_sys:: {Requet, RequestInit, RequestMode, Response}

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

pub async fn get_courses_by_teacher(teacher_id: i32) -> Result<Vec<Course>, MyError> {
    let mut opts = RequestInit::new();


    
}