use datafusion::datasource::TableProvider;
use sqlexec::environment::EnvironmentReader;
use std::sync::Arc;

use crate::execution::RGlareDbExecutionOutput;

#[derive(Debug, Clone, Copy)]
pub struct REnvironmentReader;

impl EnvironmentReader for REnvironmentReader {
    #[allow(unused_variables)]
    fn resolve_table(
        &self,
        name: &str,
    ) -> Result<Option<Arc<dyn TableProvider>>, Box<dyn std::error::Error + Send + Sync>> {
        let classes = savvy::StringSexp(
            savvy::eval_parse_text(format!(r#"base::get0(r"({name})") |> class()"#))
                .unwrap()
                .inner(),
        )
        .to_vec();

        if classes.iter().any(|&s| s == "RGlareDbExecutionOutput") {
            let sexp = savvy::Sexp(
                savvy::eval_parse_text(format!(r#"base::get0(r"({name})")$.ptr"#))
                    .unwrap()
                    .inner(),
            );
            let exec = <&RGlareDbExecutionOutput>::try_from(sexp).unwrap().clone();

            return Ok(Some(Arc::new(exec) as Arc<dyn TableProvider>));
        }

        Ok(None)
    }
}
