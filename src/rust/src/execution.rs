use crate::runtime::GLOBAL_RUNTIME;
use arrow::datatypes::Schema;
use arrow::ffi_stream::FFI_ArrowArrayStream;
use savvy::savvy;
use std::sync::{Arc, Mutex};

#[savvy]
#[derive(Clone, Debug)]
pub struct RGlareDbExecutionOutput {
    op: Arc<Mutex<glaredb::Operation>>,
}

impl From<glaredb::Operation> for RGlareDbExecutionOutput {
    fn from(opt: glaredb::Operation) -> Self {
        Self {
            op: Arc::new(Mutex::new(opt)),
        }
    }
}

#[savvy]
impl RGlareDbExecutionOutput {
    fn export_stream(&self, stream_ptr: &str) -> savvy::Result<()> {
        let stream_ptr: usize = stream_ptr.parse().unwrap();
        let stream_out = stream_ptr as *mut FFI_ArrowArrayStream;

        let mut record_stream = self.op.lock().unwrap().call();
        let batches = GLOBAL_RUNTIME
            .0
            .block_on(record_stream.to_vec())
            .expect("Must not fail"); // TODO: support async

        let schema = if batches.is_empty() {
            Arc::new(Schema::empty())
        } else {
            batches.first().unwrap().schema()
        };

        let reader =
            arrow::record_batch::RecordBatchIterator::new(batches.into_iter().map(Ok), schema);
        let reader: Box<dyn arrow::record_batch::RecordBatchReader + Send> = Box::new(reader);
        let mut stream = FFI_ArrowArrayStream::new(reader);

        unsafe {
            std::ptr::swap_nonoverlapping(
                stream_out,
                &mut stream as *mut FFI_ArrowArrayStream,
                1,
            );
        };

        Ok(())
    }
}
