use axum::Router;
use dotenvy::dotenv;
use std::{env, net::SocketAddr};

mod db;
mod models;
mod routes;
mod mq;

#[derive(Clone)]
pub struct AppState{
    pub pool: sqlx::PgPool,
    pub mq_channel: lapin::Channel
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("Error, no DATABASE_URL in .env");
    let amqp_url = env::var("AMQP_URL")
        .unwrap_or_else(|_| "amqp://guest:guest@localhost:5672".to_string());

    let pool = db::create_pool(&database_url).await;
   
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let mq_channel = mq::create_channel(&amqp_url).await;
    let state = AppState{pool: pool, mq_channel: mq_channel};

    let app = Router::new()
        .nest("/api/scores", routes::scores_router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5158")
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:5158");
    axum::serve(listener,app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
