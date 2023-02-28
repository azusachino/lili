use clap::Parser;
use std::sync::Arc;

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

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();
    let cfg_location = match args.config {
        Some(c) => c,
        None => String::from("~/.lili/cfg.yaml"),
    };

    log::debug!("config location is {}", cfg_location);

    let options = ExecOptions::from_cfg(&cfg_location);
    get_executor(Arc::new(options))
        .exec()
        .expect("fail to execute");

    Ok(())
}
