use axum::Router;
use dotenvy::dotenv;
use std::env;

mod db;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("Error, no DATABASE_URL in .env");

    let pool = db::create_pool(&database_url).await;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let app = Router::new()
        .nest("/api/scores", routes::scores_router())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5158")
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:5158");
    axum::serve(listener,app).await.unwrap();
}
