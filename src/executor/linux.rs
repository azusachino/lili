use tokio::process::Command;

use super::{async_trait, ExecOptions, Executor};

pub struct LinuxExecutor {
    options: ExecOptions,
}

impl LinuxExecutor {
    pub fn new(options: super::ExecOptions) -> Self {
        Self { options }
    }
}

#[async_trait]
impl Executor for LinuxExecutor {
    async fn exec(&self) -> anyhow::Result<()> {
        let output = Command::new("ls").arg("-la").output().await?;

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        println!("{}", self.options.cmd);
        Ok(())
    }
}
