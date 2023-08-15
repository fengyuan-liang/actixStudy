use actix_web::web;
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Teacher {
    pub id: i32, // 对应数据库的serial
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTeacher {
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

pub struct UpdateTeacher {
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

impl From<web::Data<CreateTeacher>> for CreateTeacher{
    fn from(new_teacher: Data<CreateTeacher>) -> Self {
        CreateTeacher {
            name: new_teacher.name.clone(),
            picture_url: new_teacher.picture_url.clone(),
            profile: new_teacher.profile.clone()
        }
    }
}

impl From<web::Json<UpdateTeacher>> for UpdateTeacher{
    fn from(update_teacher: Json<UpdateTeacher>) -> Self {
        UpdateTeacher {
            name: update_teacher.name.clone(),
            picture_url: update_teacher.picture_url.clone(),
            profile: update_teacher.profile.clone()
        }
    }
}