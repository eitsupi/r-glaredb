use std::sync::atomic::{AtomicU64, Ordering};

use savvy::savvy;
use tokio::runtime::Builder;

#[savvy]
pub struct RGlareDbTokioRuntime(pub tokio::runtime::Runtime);

pub static GLOBAL_RUNTIME: once_cell::sync::Lazy<RGlareDbTokioRuntime> =
    once_cell::sync::Lazy::new(|| init_glaredb().unwrap());

fn init_glaredb() -> savvy::Result<RGlareDbTokioRuntime> {
    let runtime = Builder::new_multi_thread()
        .thread_name_fn(move || {
            static THREAD_ID: AtomicU64 = AtomicU64::new(0);
            let id = THREAD_ID.fetch_add(1, Ordering::Relaxed);
            format!("glaredb-r-thread-{}", id)
        })
        .enable_all()
        .build()
        .unwrap();

    Ok(RGlareDbTokioRuntime(runtime))
}
