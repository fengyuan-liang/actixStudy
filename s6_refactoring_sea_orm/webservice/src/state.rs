use std::sync::Mutex;

use sea_orm::DatabaseConnection;
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    // pub courses: Mutex<Vec<Course>>,
    pub db: DatabaseConnection,
}