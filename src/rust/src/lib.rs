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
pub fn sql(query: &str, connection: Option<RGlareDbConnection>) -> savvy::Result<RGlareDbExecutionOutput> {
    if let Some(connection) = connection {
        connection.sql(query)
    } else {
        RGlareDbConnection::default_in_memory()?.sql(query)
    }
}
