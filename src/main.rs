use std::sync::{Arc, Mutex};

use lili::{
    executor::{get_executor, ExecOptions, Executor},
    Result,
};

fn main() -> Result<()> {
    let tokio_thread_pool = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .build()?;
    let options = ExecOptions::new("syncthing");
    // Arc for Send, Mutex for Sync
    let real_executor = Arc::new(Mutex::new(get_executor(options)));

    tokio_thread_pool.spawn(async move {
        let lock = real_executor.lock().expect("fail to get lock");
        lock.exec().await;
    });
    Ok(())
}
