use once_cell::sync::OnceCell;
use tokio::runtime;
use tokio::runtime::Runtime;

pub static RUNTIME: OnceCell<Runtime> = OnceCell::new();

pub fn rt() -> &'static Runtime {
    RUNTIME.get().expect("runtime is not initialized")
}

pub fn init() {
    match RUNTIME.get() {
        None => {
            let rt = runtime::Builder::new_multi_thread()
                .enable_time()
                .enable_io()
                .build()
                .unwrap();
            RUNTIME.set(rt).unwrap();
        }
        _ => {}
    }
}
