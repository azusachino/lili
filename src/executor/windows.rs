use std::process::Command;

use super::{ExecOptions, Executor};

pub struct WindowsExecutor {
    options: ExecOptions,
}

impl Executor for WindowsExecutor {
    fn new(options: ExecOptions) -> Self {
        Self { options }
    }

    fn exec(&self) -> anyhow::Result<()> {
        let output = Command::new(self.options.cmd.to_owned())
            // .arg(self.options.args)
            .output()?;

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        Ok(())
    }
}
