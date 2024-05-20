use crate::environment::REnvironmentReader;
use crate::error::RGlareDbError;
use crate::execution::RGlareDbExecutionOutput;
use crate::runtime::GLOBAL_RUNTIME;
use once_cell::sync::OnceCell;
use savvy::{savvy, EnvironmentSexp};
use std::sync::Arc;

#[savvy]
#[derive(Clone)]
struct RGlareDbConnection {
    pub(crate) inner: Arc<glaredb::Connection>,
}

#[savvy]
impl RGlareDbConnection {
    // TODO: support async
    pub fn default_in_memory() -> savvy::Result<RGlareDbConnection> {
        static DEFAULT_CON: OnceCell<RGlareDbConnection> = OnceCell::new();

        let con = DEFAULT_CON.get_or_try_init(|| {
            GLOBAL_RUNTIME.0.block_on(async move {
                Ok(RGlareDbConnection {
                    inner: Arc::new(
                        glaredb::ConnectOptionsBuilder::new_in_memory()
                            .environment_reader(Arc::new(REnvironmentReader::new(
                                EnvironmentSexp::global_env(),
                            )))
                            .build()?
                            .connect()
                            .await?,
                    ),
                }) as Result<_, RGlareDbError>
            })
        })?;

        Ok(con.clone())
    }

    pub fn sql(&self, query: &str) -> savvy::Result<RGlareDbExecutionOutput> {
        Ok(GLOBAL_RUNTIME
            .0
            .block_on(self.inner.sql(query).evaluate())
            .map_err(RGlareDbError::from)?
            .into())
    }
}
