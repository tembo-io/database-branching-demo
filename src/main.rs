use anyhow::Context;
use axum::{extract::Extension, routing::get, Router};
use sqlx::{PgPool, Row};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load DATABASE_URL from environment variables
    let database_url = dotenvy::var("DATABASE_URL").context("DATABASE_URL is not set")?;

    // Create a connection pool to the database
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("Failed to create connection pool")?;

    // Run migrations via sqlx
    sqlx::migrate!()
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    // Build the Axum application
    let app = Router::new()
        .route("/data", get(get_data))
        .layer(Extension(pool));

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

// Handler function to fetch data from the database
async fn get_data(Extension(pool): Extension<PgPool>) -> String {
    // Execute a query and fetch the results
    let rows = sqlx::query("SELECT id, name, email, created_at FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();

    // Convert the results to a String
    rows.iter()
        .map(|row| {
            let id: i32 = row.get(0);
            let name: String = row.get(1);
            let email: String = row.get(2);
            let created_at: chrono::NaiveDateTime = row.get(3);
            format!(
                "id: {}, name: {}, email: {}, created_at: {}",
                id, name, email, created_at
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
