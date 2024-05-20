mod connect;
mod connection;
mod environment;
mod error;
mod execution;
mod runtime;
use connection::RGlareDbConnection;
use execution::RGlareDbExecutionOutput;
use savvy::savvy;

#[savvy]
pub fn sql(query: &str, connection: RGlareDbConnection) -> savvy::Result<RGlareDbExecutionOutput> {
    connection.sql(query)
}
