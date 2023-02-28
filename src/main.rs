use clap::{Args, Parser, Subcommand};
use std::sync::Arc;

use lili::{
    executor::{get_executor, ExecOptions},
    Result, LILI_DEFAULT_EXEC_CFG,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Exec(ExecArgs),
}

#[derive(Args)]
struct ExecArgs {
    #[arg(short, long)]
    config: Option<String>,
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    match &cli.command {
        Commands::Exec(args) => {
            let cfg_location = match args.config {
                Some(ref c) => c.clone(),
                None => LILI_DEFAULT_EXEC_CFG.to_owned(),
            };

            log::debug!("config location is {}", cfg_location);

            let options = ExecOptions::from_cfg(&cfg_location);
            get_executor(Arc::new(options))
                .exec()
                .expect("fail to execute");
        }
    }

    Ok(())
}
