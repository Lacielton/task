//!
//! Creation and utilities pertaining to the database
//!
use crate::AnyResult;

use crate::SqlxPgPool;


/// Creates a database connection pool
pub async fn setup_database(url: &str) -> AnyResult<SqlxPgPool>
{
    Ok(SqlxPgPool::connect(url).await?)
}
