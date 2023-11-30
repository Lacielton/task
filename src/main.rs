//!
//!
//!
#![warn(clippy::all)]

extern crate tracing;

use sqlx::PgPool as SqlxPgPool;

use tracing_subscriber::Layer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::layer::SubscriberExt;

use axum::debug_handler;
use axum::serve as axum_serve;
use axum::Router;
use axum::extract::Json;
use axum::extract::State;
use axum::routing::get;
use axum::routing::post;

use tokio::net::TcpListener;

use serde::Serialize;
use serde::Deserialize;


#[derive(Serialize, Deserialize)]
pub struct Task
{
    pub id: i32,

    pub note: String,
    pub done: bool,
}


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


async fn root() -> &'static str
{
    "Bem-vindo à API TODO!"
}


#[debug_handler]
async fn create_task(pool: State<SqlxPgPool>, task: Json<Task>) -> Result<&'static str, String>
{
    let pool = pool.0;
    let task = task.0;

    let _ = sqlx::query!(r#"
        INSERT INTO
            tasks ("note", "done")
        VALUES
            ($1, $2)
        "#, task.note, task.done)
        .execute(&pool).await
        .map_err(|e| e.to_string())?;

    Ok("Tarefa criada")
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
        .route("/tasks", get(root))
        .route("/tasks", post(create_task))
        .with_state(sql);

    // Definir o endereço do servidor
    let addr     = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Servidor rodando em {}", addr);

    // Rodar o servidor
    axum_serve(listener, app).await
        .unwrap();
}
