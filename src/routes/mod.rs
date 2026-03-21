use axum::{
    routing::{get,post}, 
    Router
};
use sqlx::PgPool;
mod scores;

pub fn scores_router() -> Router<PgPool>{
    Router::new()
        .route("/", get(scores::get_all))
        .route("/topten",get(scores::get_top_ten))
        .route("/{id}", get(scores::get_by_id))
        .route("/",post(scores::add_score))
}