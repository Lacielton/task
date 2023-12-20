//!
//! End Project Task Service
//!

// Imports
use sqlx::PgPool as SqlxPgPool;

use envy::from_env as envy_fromenv;

use serde::Deserialize;

use anyhow::Result as AnyResult;


// Internals and exports
mod error;
mod config;

pub mod sql;
pub mod task;

pub use config::Config;
