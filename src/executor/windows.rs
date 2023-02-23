use std::process::Command;

use super::{async_trait, ExecOptions, Executor};

pub struct WindowsExecutor {
    options: ExecOptions,
}

impl WindowsExecutor {
    pub fn new(options: super::ExecOptions) -> Self {
        Self { options }
    }
}

#[async_trait]
impl Executor for WindowsExecutor {
    async fn exec(&self) -> anyhow::Result<()> {
        let output = Command::new("ls")
            // .arg(self.options.args)
            .output()?;
        println!("{:?}", self.options);

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        Ok(())
    }
}
