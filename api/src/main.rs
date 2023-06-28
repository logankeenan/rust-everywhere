mod note_service;
mod notes_model;
mod notes_routes;
mod axum_extractors;

use axum::Router;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, sqlite::SqlitePoolOptions};
use std::net::SocketAddr;
use crate::notes_routes::notes_routes;

#[derive(Clone)]
pub struct AppState {
    pool: Pool<Sqlite>
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db_location = "./sqlite.db";

    if !Sqlite::database_exists(db_location).await? {
        Sqlite::create_database(db_location).await?
    }

    let pool: Pool<Sqlite> = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_location).await?;

    sqlx::migrate!()
        .run(&pool)
        .await?;

    let state = AppState {
        pool,
    };

    let app = Router::new()
        .merge(notes_routes(state.clone()))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();


    Ok(())
}
