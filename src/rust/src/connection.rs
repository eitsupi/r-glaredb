use std::sync::Arc;

use once_cell::sync::OnceCell;
use savvy::{savvy, EnvironmentSexp};

use crate::environment::REnvironmentReader;
use crate::error::RGlareDbDatabaseError;
use crate::execution::RGlareDbExecutionOutput;
use crate::runtime;


#[savvy]
#[derive(Clone)]
struct RGlareDbConnection {
    pub(crate) inner: Arc<glaredb::Connection>,
}

impl RGlareDbConnection {
    // TODO: support async
    pub fn default_in_memory() -> savvy::Result<RGlareDbConnection> {
        static DEFAULT_CON: OnceCell<RGlareDbConnection> = OnceCell::new();

        let con = DEFAULT_CON.get_or_try_init(|| {
            runtime::block_on(async move {
                Ok::<RGlareDbConnection, savvy::Error>(RGlareDbConnection {
                    inner: Arc::new(
                        glaredb::ConnectOptionsBuilder::new_in_memory()
                            .environment_reader(Arc::new(REnvironmentReader::new(
                                EnvironmentSexp::global_env(),
                            )))
                            .build()
                            .map_err(RGlareDbDatabaseError::from)?
                            .connect()
                            .await
                            .map_err(RGlareDbDatabaseError::from)?,
                    ),
                })
            })?
        })?;

        Ok(con.clone())
    }

    pub fn sql(&self, query: &str) -> savvy::Result<RGlareDbExecutionOutput> {
        Ok(runtime::block_op(self.inner.sql(query).evaluate())?)
    }

    pub fn prql(&self, query: &str) -> savvy::Result<RGlareDbExecutionOutput> {
        Ok(runtime::block_op(self.inner.prql(query).evaluate())?)
    }

    pub fn execute(&self, query: &str) -> savvy::Result<RGlareDbExecutionOutput> {
        Ok(runtime::block_op(self.inner.execute(query).evaluate())?)
    }
}
