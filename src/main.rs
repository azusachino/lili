use clap::Parser;
use std::sync::Arc;
use tokio::sync::Mutex;

use lili::{
    executor::{get_executor, ExecOptions},
    Result,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let cfg_location = match args.config {
        Some(c) => c,
        None => String::from("~/.lili/cfg.toml"),
    };
    println!("config location is {}", cfg_location);

    let _tokio_thread_pool = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .build()?;
    let options = ExecOptions::from_cfg(&cfg_location);
    let safe_op = Arc::new(options);

    let real_executor = Arc::new(Mutex::new(get_executor(safe_op)));
    real_executor.lock().await.exec().await?;

    Ok(())
}
