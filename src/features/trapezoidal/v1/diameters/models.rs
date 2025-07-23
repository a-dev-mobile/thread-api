use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct DbModel {
    pub diameter: i32,
    pub pitch: f64,
}

#[derive(Serialize)]
pub struct ResponseModel {
    pub diameter: String,
    pub pitch: String,
    pub designation: String,
}
