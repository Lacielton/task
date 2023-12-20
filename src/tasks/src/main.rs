//!
//!
//!
#![warn(clippy::all)]

extern crate tracing;

use tracing_subscriber::Layer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::layer::SubscriberExt;

use axum::serve as axum_serve;
use axum::Router          as AxRouter;
use axum::routing::get    as AxGet;
use axum::routing::post   as AxPost;
use axum::routing::delete as AxDelete;
use axum::routing::put    as AxPut;

use tower_http::services::ServeDir as TwServeDir;

use tokio::net::TcpListener;

use anyhow::Result as AnyResult;

use tasks::Config;


#[tokio::main]
async fn main()
{
    let tracing_layer = tracing_subscriber::fmt::layer()
        .with_filter(EnvFilter::from_env("LOG"));

    tracing_subscriber::registry()
        .with(tracing_layer)
        .init();

    if let Err(e) =  run().await {
        eprintln!("{e}")
    }
}


async fn run() -> AnyResult<()>
{
    let cfg = Config::from_environment()?;
    let sql = tasks::sql::setup_database(&cfg.database_url).await?;

    // Construir o app com as rotas
    let files = TwServeDir::new("./static")
        .append_index_html_on_directories(true);

    let app = AxRouter::new()
        .route("/tasks",     AxGet(tasks::task::search))
        .route("/tasks/:id", AxGet(tasks::task::select))
        .route("/tasks",     AxPost(tasks::task::create))
        .route("/tasks/:id", AxDelete(tasks::task::delete))
        .route("/tasks/:id", AxPut(tasks::task::update))
        .nest_service("/", files)
        .with_state(sql);

    // Definir o endereço do servidor
    println!("Server listen on http://{}", cfg.host_address);

    // Rodar o servidor
    let listener = TcpListener::bind(cfg.host_address).await.unwrap();

    axum_serve(listener, app).await?;

    Ok(())
}
