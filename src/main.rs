//!
//!
//!
#![warn(clippy::all)]
mod service;
extern crate tracing;
use sqlx::PgPool as SqlxPgPool;

use tracing_subscriber::Layer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::layer::SubscriberExt;

use axum::serve as axum_serve;
use axum::Router;
use axum::routing::get    as AxGet;
use axum::routing::post   as AxPost;
use axum::routing::delete as AxDelete;
use axum::routing::put    as AxPut;

use tokio::net::TcpListener;


async fn setup_database() -> SqlxPgPool
{
    let pool = SqlxPgPool::connect("postgres://testuser:test0815@localhost:15432/tasks").await
        .unwrap();

    sqlx::query!(r#"
        CREATE TABLE IF NOT EXISTS tasks
        (
            "note" VARCHAR,
            "done" BOOLEAN
        );"#)
        .execute(&pool).await
        .unwrap();

    // Aqui você pode adicionar comandos para criar tabelas, se necessário
    pool
}


#[tokio::main]
async fn main()
{
    let tracing_layer = tracing_subscriber::fmt::layer()
        .with_filter(EnvFilter::from_env("LOG"));

    tracing_subscriber::registry()
        .with(tracing_layer)
        .init();

    let sql = setup_database().await;

    // Construir o app com as rotas
    let app = Router::new()
        .route("/tasks",        AxGet(service::task::all_tasks))
        .route("/tasks/:id",    AxGet(service::task::select))
        .route("/tasks",        AxPost(service::task::create))
        .route("/tasks/:id",    AxDelete(service::task::delete))
        .route("/tasks/:id",    AxPut(service::task::update))
        .with_state(sql);

    // Definir o endereço do servidor
    let addr     = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server listen in {}", addr);

    // Rodar o servidor
    axum_serve(listener, app).await
        .unwrap();
}
