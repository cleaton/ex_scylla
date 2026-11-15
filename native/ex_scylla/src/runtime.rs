use std::sync::OnceLock;
use tokio::runtime;
use tokio::runtime::Runtime;

pub static RUNTIME: OnceLock<Runtime> = OnceLock::new();

pub fn rt() -> &'static Runtime {
    RUNTIME.get().expect("runtime is not initialized")
}

pub fn init(worker_threads: Option<usize>) {
    RUNTIME.get_or_init(|| {
        let mut builder = runtime::Builder::new_multi_thread();
        if let Some(n) = worker_threads.filter(|n| *n > 0) {
            builder.worker_threads(n);
        }
        builder.enable_time().enable_io().build().unwrap()
    });
}
