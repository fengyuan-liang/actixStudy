use actix_web::web;
use actix_web::web::{Data, Json};
use log::info;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Teacher {
    pub id: i32, // 对应数据库的serial
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}

impl From<web::Json<Teacher>> for Teacher {
    fn from(value: web::Json<Teacher>) -> Self {
        Teacher {
            id: value.id,
            name: value.name.clone(),
            picture_url: value.picture_url.clone(),
            profile: value.profile.clone(),
        }
    }
}

#[derive(Deserialize, Debug, Clone, sqlx::FromRow)]
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

impl From<Json<CreateTeacher>> for CreateTeacher {
    fn from(new_teacher: Json<CreateTeacher>) -> Self {
        info!("{:?}", new_teacher);
        CreateTeacher {
            name: new_teacher.name.clone(),
            picture_url: new_teacher.picture_url.clone(),
            profile: new_teacher.profile.clone(),
        }
    }
}

impl From<Json<UpdateTeacher>> for UpdateTeacher {
    fn from(update_teacher: Json<UpdateTeacher>) -> Self {
        UpdateTeacher {
            name: update_teacher.name.clone(),
            picture_url: update_teacher.picture_url.clone(),
            profile: update_teacher.profile.clone(),
        }
    }
}
