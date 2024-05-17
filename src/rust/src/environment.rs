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
            let stream_reader = ArrowArrayStreamReader::from_r(sexp).unwrap();
            let schema = stream_reader.schema();
            let batches =
                stream_reader.collect::<Result<Vec<RecordBatch>, arrow::error::ArrowError>>()?;
            let table = MemTable::try_new(schema, vec![batches])?;

            return Ok(Some(Arc::new(table) as Arc<dyn TableProvider>));
        }

        Ok(None)
    }
}

trait FromRArrowArrayStream: Sized {
    fn from_r(sexp: savvy::Sexp) -> Result<Self, savvy::Error>;
}

// Import nanoarrow_array_stream
// Copied from https://github.com/JosiahParry/arrow-extendr/blob/1ff628cd5e9c208c1aff99bc8aa92a3b7b9303dc/src/from.rs#L286-L302
impl FromRArrowArrayStream for ArrowArrayStreamReader {
    fn from_r(sexp: savvy::Sexp) -> Result<Self, savvy::Error> {
        if sexp
            .get_class()
            .unwrap_or_default()
            .iter()
            .all(|&s| s != "nanoarrow_array_stream")
        {
            return Err(savvy::Error::from("Not a nanoarrow_array_stream"));
        }

        let func = savvy::FunctionSexp(
            savvy::eval_parse_text(r#"nanoarrow::nanoarrow_pointer_export"#)
                .unwrap()
                .inner(),
        );

        let stream = FFI_ArrowArrayStream::empty();
        let stream_ptr = &stream as *const FFI_ArrowArrayStream as usize;
        let mut args = savvy::FunctionArgs::new();
        args.add("ptr_src", sexp)?;
        args.add("ptr_dst", stream_ptr.to_string())?;

        let _ = func.call(args)?;

        ArrowArrayStreamReader::try_new(stream).map_err(|e| savvy::Error::from(e.to_string()))
    }
}
