use datafusion::arrow::error::ArrowError;
use glaredb::DataFusionError;
use metastore::errors::MetastoreError;
use sqlexec::errors::ExecError;
use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum RGlareDbError {
    #[error(transparent)]
    Arrow(#[from] ArrowError),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    DataFusion(#[from] DataFusionError),

    #[error(transparent)]
    Metastore(#[from] MetastoreError),
    #[error(transparent)]
    Exec(#[from] ExecError),
    #[error(transparent)]
    ConfigurationBuilder(#[from] glaredb::ConnectOptionsBuilderError),

    #[error("{0}")]
    Other(String),
}

impl RGlareDbError {
    pub fn new(msg: impl Display) -> Self {
        Self::Other(msg.to_string())
    }
}

impl From<RGlareDbError> for savvy::Error {
    fn from(err: RGlareDbError) -> Self {
        savvy::Error::new(&err.to_string())
    }
}
