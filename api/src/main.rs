mod note_service;
mod notes_model;
mod notes_routes;
mod axum_extractors;

use axum::Router;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use std::net::SocketAddr;
use axum::routing::get;
use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};
use sqlx::migrate::MigrateDatabase;
use crate::notes_routes::notes_routes;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct AppState {
    pool: Pool<Sqlite>,
}

async fn root() -> &'static str {
    "Hello World!"
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    Logger::try_with_str("info").unwrap()
        .log_to_file(
            FileSpec::default()
        )
        .rotate(Criterion::Size(10000000),
                Naming::Timestamps,
                Cleanup::KeepLogFiles(7),
        )
        .start().unwrap();

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
        .route("/", get(root))
        .merge(notes_routes(state.clone()))
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_origin([
                    "http://localhost:3002".parse().unwrap(),
                    "http://localhost:4000".parse().unwrap(),
                    "https://rust-everywhere-spa.logankeenan.com".parse().unwrap()
                ])
                .allow_headers(Any)
        )
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();


    Ok(())
}
