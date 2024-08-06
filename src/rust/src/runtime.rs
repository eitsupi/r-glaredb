use std::sync::atomic::{AtomicU64, Ordering};

use once_cell::sync::OnceCell;
use tokio::runtime::{Builder, Runtime};

use crate::error::RGlareDbDatabaseError;
use crate::RGlareDbExecutionOutput;

static GLOBAL_RUNTIME: once_cell::sync::OnceCell<Runtime> = OnceCell::new();

pub fn block_op<
    F: std::future::Future<Output = Result<glaredb::Operation, glaredb::DatabaseError>>,
>(
    future: F,
) -> Result<RGlareDbExecutionOutput, savvy::Error> {
    Ok(GLOBAL_RUNTIME
        .get_or_try_init(init_runtime)?
        .block_on(future)
        .map_err(RGlareDbDatabaseError::from)?
        .into())
}

pub fn block_on<F: std::future::Future>(future: F) -> Result<F::Output, savvy::Error> {
    Ok(GLOBAL_RUNTIME
        .get_or_try_init(init_runtime)?
        .block_on(future))
}


fn init_runtime() -> Result<Runtime, RGlareDbDatabaseError> {
    Ok(Builder::new_multi_thread()
        .thread_name_fn(move || {
            static THREAD_ID: AtomicU64 = AtomicU64::new(0);
            let id = THREAD_ID.fetch_add(1, Ordering::Relaxed);
            format!("glaredb-r-thread-{}", id)
        })
        .enable_all()
        .build()?)
}
