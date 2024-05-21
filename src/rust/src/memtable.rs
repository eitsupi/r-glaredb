use arrow::array::RecordBatchReader;
use arrow::ffi_stream::{ArrowArrayStreamReader, FFI_ArrowArrayStream};
use datafusion::arrow::array::RecordBatch;
use datafusion::datasource::MemTable;
use savvy::savvy;
use std::sync::Arc;

#[savvy]
struct RGlareDbMemTable {
    pub(crate) inner: Arc<MemTable>,
}

#[savvy]
impl RGlareDbMemTable {
    pub fn import_stream(stream_ptr: savvy::Sexp) -> savvy::Result<Self> {
        let stream_reader = unsafe {
            let stream = savvy::ExternalPointerSexp::try_from(stream_ptr)?
                .cast_mut_unchecked::<FFI_ArrowArrayStream>();
            ArrowArrayStreamReader::from_raw(stream).map_err(|e| e.to_string())?
        };
        let schema = stream_reader.schema();
        let batches = stream_reader
            .collect::<Result<Vec<RecordBatch>, arrow::error::ArrowError>>()
            .map_err(|e| e.to_string())?;
        let table = MemTable::try_new(schema, vec![batches]).map_err(|e| e.to_string())?;

        Ok(RGlareDbMemTable {
            inner: Arc::new(table),
        })
    }
}

impl From<MemTable> for RGlareDbMemTable {
    fn from(table: MemTable) -> Self {
        Self {
            inner: Arc::new(table),
        }
    }
}
