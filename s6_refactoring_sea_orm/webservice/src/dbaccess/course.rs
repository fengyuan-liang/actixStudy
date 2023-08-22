use log::error;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use sea_orm::ActiveValue::Set;

use crate::error::MyError;
use crate::models::course;
use crate::models::course::Model;
use crate::models::prelude::Course;

pub async fn get_all_courses_for_db(pool: &DatabaseConnection) -> Result<Vec<Model>, MyError> {
    Course::find()
        .all(pool)
        .await
        .map_err(|err| {
            error!("{}", err.to_string());
            MyError::DBError(format!("query err:{}", err.to_string()))
        })
}


pub async fn get_courses_for_teacher_db(pool: &DatabaseConnection, teacher_id: i32) -> Result<Vec<Model>, MyError> {
    // let rows: Vec<Course> = sqlx::query_as!(
    //     Course,
    //     r#"SELECT * FROM course
    //     WHERE teacher_id = $1"#,
    //     teacher_id
    // )
    //     .fetch_all(pool)
    //     .await?;
    //
    // Ok(rows)
    Course::find()
        .filter(course::Column::TeacherId.eq(teacher_id))
        .all(pool)
        .await
        .map_err(|err| MyError::DBError(format!("get_courses_for_teacher_db err:{}", err.to_string())))
}


pub async fn get_course_details_db(pool: &DatabaseConnection, teacher_id: i32, course_id: i32) -> Result<Vec<Model>, MyError> {
    // let row = sqlx::query_as!(
    //     Course,
    //     r#"SELECT *
    //         FROM course
    //         WHERE teacher_id = $1 and id = $2"#,
    //     teacher_id,
    //     course_id
    // )
    //     .fetch_optional(pool)
    //     .await?
    //     .ok_or(MyError::NotFound("Course Id not found".into()));
    // row
    // if let Some(course) = row {
    //     Ok(course)
    // } else {
    //     Err(MyError::NotFound("Course Id not found".into()))
    // }
    Course::find()
        .filter(course::Column::TeacherId.eq(teacher_id))
        .filter(course::Column::Id.eq(course_id))
        .all(pool)
        .await
        .map_err(|err| MyError::DBError(format!("get_courses_for_teacher_db err:{}", err.to_string())))
}

pub async fn post_new_course_db(pool: &DatabaseConnection, course_name: String) -> Result<i32, MyError> {
    // let row = sqlx::query_as!(
    //     Course,
    //     r#"INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level)
    //     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    //     RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level"#,
    //     new_course.teacher_id, new_course.name, new_course.description,
    //     new_course.format, new_course.structure, new_course.duration,
    //     new_course.price, new_course.language, new_course.level
    //     )
    //     .fetch_one(pool)
    //     .await?;
    // Ok(row)
    let new_course = course::ActiveModel {
        id: Default::default(),
        teacher_id: Default::default(),
        name: ActiveValue::Set(course_name),
        time: Default::default(),
        description: Default::default(),
        format: Default::default(),
        structure: Default::default(),
        duration: Default::default(),
        price: Default::default(),
        language: Default::default(),
        level: Default::default(),
    };
    Course::insert(new_course)
        .exec(pool)
        .await
        .map(|r| r.last_insert_id)
        .map_err(|err| MyError::DBError(format!("get_courses_for_teacher_db err:{}", err.to_string())))
}


pub async fn delete_course_db(pool: &DatabaseConnection, teacher_id: i32, id: i32) -> Result<String, MyError> {
    // let course_row = sqlx::query!(
    //     "DELETE FROM course where teacher_id = $1 and id=$2",
    //     teacher_id,
    //     id,
    // )
    //     .execute(pool)
    //     .await?;
    // Ok(format!("DeletedI{:?}record", course_row))
    let option = Course::find()
        .filter(course::Column::TeacherId.eq(teacher_id))
        .filter(course::Column::Id.eq(id))
        .one(pool)
        .await;
    let option:Model = option.expect("xx").unwrap();
    option.delete(pool)
        .await
        .map(|ele| format!("DeletedI{:?}record", ele.rows_affected))
        .map_err(|err| MyError::DBError(format!("DeletedI{:?}record", err.to_string())))
}

pub async fn update_course_details_db(
    pool: &DatabaseConnection,
    teacher_id: i32,
    id: i32,
    update_course: Model,
) -> Result<Model, MyError> {
    // 先查到
    let option = Course::find()
        .filter(course::Column::Id.eq(id))
        .filter(course::Column::TeacherId.eq(teacher_id))
        .one(pool)
        .await
        .unwrap();
    // into activeModel
    let mut c: course::ActiveModel = option.unwrap().into();
    if c.description.clone().unwrap() != update_course.description {
        c.description = Set(update_course.description)
    }
    c.update(pool)
        .await
        .map(|ele| ele)
        .map_err(|err| MyError::DBError(format!("DeletedI{:?}record", err.to_string())))
    // let current_course_row = sqlx::query_as!(
    //     Course,
    //     "SELECT * FROM course where teacher_id=$1 and id=$2",
    //     teacher_id,
    //     id
    // )
    //     .fetch_one(pool)
    //     .await
    //     .map_err(|_err| MyError::NotFound("Course Id not found".into()))?;
    //
    // let name: String = if let Some(name) = update_course.name {
    //     name
    // } else {
    //     current_course_row.name
    // };
    // let description: String = if let Some(description) = update_course.description {
    //     description
    // } else {
    //     current_course_row
    //         .description
    //         .unwrap_or_default()
    // };
    // let format: String = if let Some(format) = update_course.format {
    //     format
    // } else {
    //     current_course_row
    //         .format
    //         .unwrap_or_default()
    // };
    // let structure: String = if let Some(structure) = update_course.structure {
    //     structure
    // } else {
    //     current_course_row
    //         .structure
    //         .unwrap_or_default()
    // };
    // let duration: String = if let Some(duration) = update_course.duration {
    //     duration
    // } else {
    //     current_course_row
    //         .duration
    //         .unwrap_or_default()
    // };
    // let level: String = if let Some(level) = update_course.level {
    //     level
    // } else {
    //     current_course_row
    //         .level
    //         .unwrap_or_default()
    // };
    // let language: String = if let Some(language) = update_course.language {
    //     language
    // } else {
    //     current_course_row
    //         .language
    //         .unwrap_or_default()
    // };
    // let price: i32 = if let Some(price) = update_course.price {
    //     price
    // } else {
    //     current_course_row
    //         .price
    //         .unwrap_or_default()
    // };
    // let course_row = sqlx::query_as!(
    //     Course,
    //     "UPDATE course SET name = $1, description = $2, format = $3,
    //         structure = $4, duration = $5, price = $6, language = $7,
    //         level = $8 where teacher_id = $9 and id = $10
    //         RETURNING id, teacher_id, name, time,
    //         description, format, structure,duration, price, language, level",
    //     name,
    //     description,
    //     format,
    //     structure,
    //     duration,
    //     price,
    //     language,
    //     level,
    //     teacher_id,
    //     id
    // )
    //     .fetch_one(pool)
    //     .await;
    // if let Ok(course) = course_row {
    //     Ok(course)
    // } else {
    //     Err(MyError::NotFound("Course id not found".into()))
    // }
}