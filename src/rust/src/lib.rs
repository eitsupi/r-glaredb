mod connect;
mod connection;
mod environment;
mod error;
mod execution;
mod runtime;
mod table;
use connection::RGlareDbConnection;
use execution::RGlareDbExecutionOutput;
use savvy::savvy;

/// @rdname glaredb_execute
/// @export
/// @order 2
#[savvy]
pub fn glaredb_sql(
    query: &str,
    connection: Option<&RGlareDbConnection>,
) -> savvy::Result<RGlareDbExecutionOutput> {
    if let Some(connection) = connection {
        connection.sql(query)
    } else {
        RGlareDbConnection::default_in_memory()?.sql(query)
    }
}

/// @rdname glaredb_execute
/// @export
/// @order 3
#[savvy]
pub fn glaredb_prql(
    query: &str,
    connection: Option<&RGlareDbConnection>,
) -> savvy::Result<RGlareDbExecutionOutput> {
    if let Some(connection) = connection {
        connection.prql(query)
    } else {
        RGlareDbConnection::default_in_memory()?.prql(query)
    }
}

#[savvy]
pub fn execute(
    query: &str,
    connection: Option<&RGlareDbConnection>,
) -> savvy::Result<RGlareDbExecutionOutput> {
    if let Some(connection) = connection {
        connection.execute(query)
    } else {
        RGlareDbConnection::default_in_memory()?.execute(query)
    }
}
