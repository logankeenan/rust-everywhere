mod note_service;
mod notes_model;
mod notes_routes;
mod axum_extractors;

use axum::Router;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use std::net::SocketAddr;
use crate::notes_routes::notes_routes;

#[derive(Clone)]
pub struct AppState {
    pool: Pool<Sqlite>
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool: Pool<Sqlite> = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("./sqlite.db").await?;

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
