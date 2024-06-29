use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum RGlareDbDatabaseError {
    #[error("{0}")]
    Database(#[from] glaredb::DatabaseError),
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
