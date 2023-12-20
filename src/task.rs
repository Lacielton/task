use serde::Deserialize;
use serde::Serialize;

use axum::extract::Json;
use axum::extract::State;
use axum::extract::Path as AxPath;
use axum::debug_handler;

use sqlx::PgPool as SqlxPgPool;


#[derive(Debug, Deserialize, Serialize)]
pub struct Task
{
    pub id: i32,

    pub note: String,
    pub done: bool,
}


#[derive(Debug, Deserialize)]
pub struct NewTask
{
    pub note: String,
}


#[derive(Debug, Deserialize)]
pub struct UpdateTask
{
    pub note: String,
    pub done: bool,
}


#[debug_handler]
pub async fn create(pool: State<SqlxPgPool>, task: Json<Task>) -> Result<&'static str, String>
{
    let pool = pool.0;
    let task = task.0;

    sqlx::query!(r#"
        INSERT INTO
            tasks.tasks ("note", "done")
        VALUES
            ($1, $2)
        "#, task.note, task.done)
        .execute(&pool).await
        .map_err(|e| e.to_string())?;

    Ok("Task Create")
}


#[debug_handler]
pub async fn select(pool: State<SqlxPgPool>, task:AxPath<i32>) -> Result<&'static str, String>
{
    let pool = pool.0;
    let task = task.0;

    let _task = sqlx::query_as!(Task, r#"
        SELECT
             *
        FROM
            tasks.tasks
        WHERE
            id = $1
        "#, task)
        .fetch_optional(&pool).await
        .map_err(|e| e.to_string())?;

    Ok("Selected Task")
}


#[debug_handler]
pub async fn search(pool: State<SqlxPgPool>) -> Result<Json<Vec<Task>>, String>
{
    let pool = pool.0;

    let task = sqlx::query_as!(Task, r#"
        SELECT
             *
        FROM
            tasks.tasks
        "#)
        .fetch_all(&pool).await
        .map_err(|e| e.to_string())?;

    Ok(Json(task))
}


#[debug_handler]
pub async fn update(pool: State<SqlxPgPool>, AxPath(id): AxPath<i32>, Json(task): Json<UpdateTask>) -> Result<&'static str, String>
{
    let pool = pool.0;
    let task = task.note;

    let _task = sqlx::query_as!(Task, r#"
        UPDATE
           tasks.tasks
        SET
            note = $1
        WHERE
            id = $2
        "#, task, id)
        .execute(&pool).await
        .map_err(|e| e.to_string())?;

    Ok("Update Task")

}


#[debug_handler]
pub async fn delete(pool: State<SqlxPgPool>, task: AxPath<i32>) -> Result<&'static str, String>
{
    let pool = pool.0;
    let task = task.0;

        let _task = sqlx::query!(r#"
        DELETE FROM
            tasks.tasks
        WHERE
            id = $1
        "#, task )
        .execute(&pool).await
        .map_err(|e| e.to_string())?;

    Ok("Task Deleted")
}
