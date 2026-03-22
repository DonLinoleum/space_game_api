use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json
};
use sqlx::PgPool;
use crate::models::{CreateScore, Score};

pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<Score>>, StatusCode>
{
    let scores = sqlx::query_as::<_,Score>("SELECT * FROM scores ORDER BY id DESC LIMIT 100")
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(scores))
}

pub async fn get_top_ten(State(pool):State<PgPool>) -> Result<Json<Vec<Score>>, StatusCode>
{
     let scores = sqlx::query_as::<_,Score>("SELECT * FROM scores ORDER BY scores DESC LIMIT 10")
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(scores))
}

pub async fn get_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> Result<Json<Score>, StatusCode>
{
    let score = sqlx::query_as::<_,Score>("SELECT * FROM scores WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match score{
        Some(s) => Ok(Json(s)),
        None => Err(StatusCode::NOT_FOUND)
    }
}

pub async fn add_score(State(pool):State<PgPool>, headers:HeaderMap, Json(data): Json<CreateScore>) -> Result<Json<Score>, StatusCode>
{
    if !headers.contains_key("Space-Game"){
        return Err(StatusCode::BAD_REQUEST);
    }

    let score = sqlx::query_as::<_,Score>("INSERT INTO scores (name, level, scores, created) VALUES ($1,$2,$3,NOW()) RETURNING *")
        .bind(data.name)
        .bind(data.level)
        .bind(data.scores)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(score))
}