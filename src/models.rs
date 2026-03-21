use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Score{
    pub id: i32,
    pub name: Option<String>,
    pub level: i32,
    pub scores: i32,
    pub created: Option<DateTime<Utc>>
}

#[derive(Debug, Deserialize)]
pub struct CreateScore{
    pub name: Option<String>,
    pub level: i32,
    pub scores: i32
}