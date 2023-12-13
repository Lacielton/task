use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Task
{
    pub id: i32,

    pub note: String,
    pub done: bool,
}