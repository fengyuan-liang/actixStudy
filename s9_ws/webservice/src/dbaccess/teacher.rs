use actix_web::web;
use log::info;
use sqlx::postgres::PgPool;

use crate::error::MyError;
use crate::models::course::Course;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};

pub async fn get_all_teachers_db(pool: &PgPool) -> Result<Vec<Teacher>, MyError> {
    let rows = sqlx::query!(
        r#"SELECT id, name, picture_url, profile
            FROM teacher"#,
    )
        .fetch_all(pool)
        .await?;

    let teachers:Vec<Teacher> = rows
        .iter()
        .map(|r| Teacher {
            id: r.id,
            name: r.name.clone(),
            picture_url: r.picture_url.clone(),
            profile: r.profile.clone().unwrap(),
        })
        .collect();

    match teachers.len() {
        0 => Err(MyError::NotFound("No teachers found".into())),
        _ => Ok(teachers),
    }
}

pub async fn get_teacher_details_db(pool: &PgPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        r#"select id, name, picture_url, profile from teacher where id=$1"#,
        teacher_id,
    )
        .fetch_one(pool)
        .await
        .map(|r| Teacher {
            id: r.id,
            name: r.name,
            picture_url: r.picture_url,
            profile: r.profile.clone().unwrap(),
        })
        .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;
    Ok(row)
}

pub async fn post_new_teacher_db(
    pool: &PgPool,
    new_teacher: CreateTeacher,
) -> Result<Teacher, MyError> {
    info!("CreateTeacher:[{:?}]", new_teacher);
    let row = sqlx::query!(
        r#"insert into teacher(name, picture_url, profile)
        values ($1, $2, $3) returning id, name, picture_url, profile"#,
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile,
    )
        .fetch_one(pool)
        .await?;

    Ok(Teacher {
        id: row.id,
        name: row.name,
        picture_url: row.picture_url,
        profile: row.profile.clone().unwrap(),
    })
}

pub async fn update_teacher_details_db(
    pool: &PgPool,
    teacher_id: i32,
    update_teacher: UpdateTeacher,
) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        r#"select id, name, picture_url, profile from teacher where id = $1"#,
        teacher_id
    )
        .fetch_one(pool)
        .await
        .map_err(|_err| MyError::NotFound("Teacher id not found".into()))?;
    // 后面就是拿着比较下 如果值不一样就跟新db
    Ok(Teacher{
        id: 0,
        name: "".to_string(),
        picture_url: "".to_string(),
        profile: "".to_string(),
    })
}

pub async fn delete_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<String, MyError> {
    let row = sqlx::query(&format!("delete from teacher where id = {}", teacher_id))
        .execute(pool)
        .await
        .map_err(|_err| MyError::DBError("unable to delete teacher".into()))?;
    Ok(format!("delete {:?} record", row))
}
