use clap::Parser;

use lili::{
    cmds::{Commands, ExecOptions, Executor},
    Result, LILI_DEFAULT_EXEC_CFG,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    match &cli.command {
        Commands::Exec(args) => {
            let cfg_location = match args.config {
                Some(ref c) => shellexpand::tilde(c).to_string(),
                None => LILI_DEFAULT_EXEC_CFG.to_owned(),
            };

            tracing::debug!("config location is {}", cfg_location);

            let options = ExecOptions::from_cfg(&cfg_location)?;
            Executor::new(options).exec().await?;
        }
    }

    Ok(())
}
