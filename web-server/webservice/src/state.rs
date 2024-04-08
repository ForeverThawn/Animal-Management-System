use sqlx::postgres::PgPool;

use std::sync::Mutex;
use super::models::Animal;
use super::models::LoginData;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,

    // pub animals: Mutex<Vec<Animal>>,
    pub db: PgPool,
}
