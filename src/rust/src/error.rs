use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum RGlareDbDatabaseError {
    #[error(transparent)]
    Database(#[from] glaredb::DatabaseError),

    #[error(transparent)]
    Connection(#[from] glaredb::ConnectOptionsBuilderError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl RGlareDbDatabaseError {
    pub fn new(msg: impl Display) -> Self {
        Self::Database(glaredb::DatabaseError::new(msg.to_string()))
    }
}

impl From<RGlareDbDatabaseError> for savvy::Error {
    fn from(err: RGlareDbDatabaseError) -> Self {
        savvy::Error::new(&err.to_string())
    }
}
