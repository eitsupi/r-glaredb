use crate::execution::RGlareDbExecutionOutput;
use arrow::array::RecordBatchReader;
use arrow::ffi_stream::{ArrowArrayStreamReader, FFI_ArrowArrayStream};
use datafusion::arrow::array::RecordBatch;
use datafusion::datasource::{MemTable, TableProvider};
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

        if classes
            .iter()
            .any(|&s| s == "RPolarsDataFrame" || s == "ArrowTabular")
        {
            let sexp = savvy::Sexp(
                savvy::eval_parse_text(format!(
                    r#"base::get0(r"({name})") |> nanoarrow::as_nanoarrow_array_stream()"#
                ))
                .unwrap()
                .inner(),
            );
            let stream_ptr = savvy::ExternalPointerSexp::try_from(sexp).unwrap();
            let stream = unsafe { stream_ptr.cast_mut_unchecked::<FFI_ArrowArrayStream>() };
            let stream_reader = unsafe { ArrowArrayStreamReader::from_raw(stream).unwrap() };
            let schema = stream_reader.schema();
            let batches =
                stream_reader.collect::<Result<Vec<RecordBatch>, arrow::error::ArrowError>>()?;
            let table = MemTable::try_new(schema, vec![batches])?;

            return Ok(Some(Arc::new(table) as Arc<dyn TableProvider>));
        }

        Ok(None)
    }
}
