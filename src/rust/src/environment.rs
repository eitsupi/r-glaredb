use datafusion::datasource::TableProvider;
use sqlexec::environment::EnvironmentReader;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct REnvironmentReader;

impl EnvironmentReader for REnvironmentReader {
    #[allow(unused_variables)]
    fn resolve_table(
        &self,
        name: &str,
    ) -> Result<Option<Arc<dyn TableProvider>>, Box<dyn std::error::Error + Send + Sync>> {
        // TODO: implement this
        Ok(None)
    }
}
