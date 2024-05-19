mod connect;
mod connection;
mod environment;
mod error;
mod execution;
mod runtime;
use connection::RGlareDbConnection;
use execution::RGlareDbExecutionOutput;
use savvy::{savvy, EnvironmentSexp};

#[savvy]
pub fn sql_(query: &str, env: Option<EnvironmentSexp>) -> savvy::Result<RGlareDbExecutionOutput> {
    let env = env.unwrap_or(EnvironmentSexp::global_env());

    RGlareDbConnection::default_in_memory(env)?.sql(query)
}
