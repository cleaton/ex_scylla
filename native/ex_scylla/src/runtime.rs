use std::sync::OnceLock;
use tokio::runtime;
use tokio::runtime::Runtime;

pub static RUNTIME: OnceLock<Runtime> = OnceLock::new();

pub fn rt() -> &'static Runtime {
    RUNTIME.get().expect("runtime is not initialized")
}

pub fn init() {
    RUNTIME.get_or_init(|| {
        runtime::Builder::new_multi_thread()
            .enable_time()
            .enable_io()
            .build()
            .unwrap()
    });
}
