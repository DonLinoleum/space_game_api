use std::net::SocketAddr;

use axum::{
    Json, extract::{ConnectInfo, Path, State}, http::{HeaderMap, StatusCode}
};
use chrono::Utc;
use serde_json::json;
use lapin::{BasicProperties, options::BasicPublishOptions};
use crate::{AppState, models::{CreateScore, Score}};

pub async fn get_all(State(state): State<AppState>) -> Result<Json<Vec<Score>>, StatusCode>
{
    let scores = sqlx::query_as::<_,Score>("SELECT * FROM scores ORDER BY id DESC LIMIT 100")
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(scores))
}

pub async fn get_top_ten(State(state):State<AppState>) -> Result<Json<Vec<Score>>, StatusCode>
{
     let scores = sqlx::query_as::<_,Score>("SELECT * FROM scores ORDER BY scores DESC LIMIT 10")
    .fetch_all(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(scores))
}

pub async fn get_by_id(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<Score>, StatusCode>
{
    let score = sqlx::query_as::<_,Score>("SELECT * FROM scores WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match score{
        Some(s) => Ok(Json(s)),
        None => Err(StatusCode::NOT_FOUND)
    }
}

pub async fn add_score(
    ConnectInfo(addr): ConnectInfo<SocketAddr>, 
    State(state):State<AppState>, 
    headers:HeaderMap, 
    Json(data): Json<CreateScore>) -> Result<Json<Score>, StatusCode>
{
    if !headers.contains_key("Space-Game"){
        return Err(StatusCode::BAD_REQUEST);
    }

    let score = sqlx::query_as::<_,Score>("INSERT INTO scores (name, level, scores, created) VALUES ($1,$2,$3,NOW()) RETURNING *")
        .bind(data.name)
        .bind(data.level)
        .bind(data.scores)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let payload = json!({
        "ip": addr.ip().to_string(),
        "date": Utc::now().to_rfc3339(),
        "score": score.scores,
        "level": score.level,
        "name": score.name
    });

   let _ = state.mq_channel.basic_publish(
    "".into(), 
    "score_logs".into(), 
    BasicPublishOptions::default(),
     payload.to_string().as_bytes(), 
     BasicProperties::default())
        .await.expect("Failed to publish message");

    Ok(Json(score))
}
