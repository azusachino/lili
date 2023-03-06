mod executor;

pub use executor::{ExecOptions, Executor};

use clap::{Args, Subcommand};

#[derive(Subcommand)]
pub enum Commands {
    Exec(ExecArgs),
}

#[derive(Args)]
pub struct ExecArgs {
    #[arg(short, long)]
    pub config: Option<String>,
}
