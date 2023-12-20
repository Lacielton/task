//!
//! Configuration Loading
//!
use crate::envy_fromenv;

use crate::AnyResult;

use crate::Deserialize;

use std::net::SocketAddr;


#[derive(Debug, Deserialize)]
pub struct Config
{
    /// Address to listen on for incoming connections for the RESTful service
    pub host_address: SocketAddr,

    /// URL to the SQL database used as backend
    pub database_url: String,
}


impl Config
{
    pub fn from_environment() -> AnyResult<Self>
    {
        Ok(envy_fromenv()?)
    }
}
