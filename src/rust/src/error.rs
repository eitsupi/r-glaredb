#[derive(Debug, thiserror::Error)]
pub enum RGlareDbDatabaseError {
    #[error(transparent)]
    Database(#[from] glaredb::DatabaseError),

    #[error(transparent)]
    Connection(#[from] glaredb::ConnectOptionsBuilderError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
