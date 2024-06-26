use arrow::array::RecordBatchReader;
use arrow::datatypes::Schema;
use arrow::ffi_stream::{ArrowArrayStreamReader, FFI_ArrowArrayStream};
use datafusion::arrow::array::RecordBatch;
use datafusion::datasource::MemTable;
use savvy::savvy;
use std::sync::Arc;

#[savvy]
struct RGlareDbTable {
    pub schema: Arc<Schema>,
    pub batches: Vec<RecordBatch>,
}

#[savvy]
impl RGlareDbTable {
    fn print(&self) -> savvy::Result<()> {
        let schema = &self.schema;
        let batches = &self.batches;

        let disp = arrow_util::pretty::pretty_format_batches(
            &schema,
            &batches,
            Some(terminal_util::term_width()),
            None,
        )
        .map_err(|e| e.to_string())?;

        savvy::r_println!("{disp}");

        Ok(())
    }

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

        Ok(RGlareDbTable { schema, batches })
    }

    pub fn export_stream(&self, stream_ptr: savvy::Sexp) -> savvy::Result<()> {
        let stream_out = unsafe {
            savvy::ExternalPointerSexp::try_from(stream_ptr)?
                .cast_mut_unchecked::<FFI_ArrowArrayStream>()
        };

        let reader = arrow::record_batch::RecordBatchIterator::new(
            self.batches.clone().into_iter().map(Ok),
            self.schema.clone(),
        );

        let reader: Box<dyn arrow::record_batch::RecordBatchReader + Send> = Box::new(reader);
        let mut stream = FFI_ArrowArrayStream::new(reader);

        unsafe {
            std::ptr::swap_nonoverlapping(stream_out, &mut stream as *mut FFI_ArrowArrayStream, 1);
        };

        Ok(())
    }
}

impl TryFrom<&RGlareDbTable> for MemTable {
    type Error = String;

    fn try_from(table: &RGlareDbTable) -> Result<MemTable, String> {
        MemTable::try_new(table.schema.clone(), vec![table.batches.clone()])
            .map_err(|e| e.to_string())
    }
}
