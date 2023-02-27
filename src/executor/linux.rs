use std::{process::Command, sync::Arc};

use super::{async_trait, ExecOptions, Executor};
use tokio::task;

pub struct LinuxExecutor {
    options: Arc<ExecOptions>,
}

impl LinuxExecutor {
    pub fn new(options: Arc<ExecOptions>) -> Self {
        Self { options }
    }
}

#[async_trait]
impl Executor for LinuxExecutor {
    async fn exec(&self) -> anyhow::Result<()> {
        let cmds = self.options.processes.lock().expect("fail to read options");

        let mut handles = Vec::new();
        for (_, v) in cmds.iter() {
            let cmd = String::clone(&v.cmd);
            let args = Option::clone(&v.args);
            let mut real_cmd = Command::new(cmd);
            if args.is_some() {
                real_cmd.arg(args.unwrap());
            }
            println!("{:?}", real_cmd);
            handles.push(task::spawn(async move {
                let output = real_cmd.output().unwrap();
                println!("{:?}", output.status);
                println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            }));
        }

        Ok(())
    }
}
