use std::{process::Command, sync::Arc};

use super::{async_trait, ExecOptions, Executor};

pub struct WindowsExecutor {
    options: Arc<ExecOptions>,
}

impl WindowsExecutor {
    pub fn new(options: Arc<ExecOptions>) -> Self {
        Self { options }
    }
}

#[async_trait]
impl Executor for WindowsExecutor {
    async fn exec(&self) -> anyhow::Result<()> {
        let cmds = self.options.processes.lock().expect("fail to read options");
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_io()
            .thread_name("tokio-")
            .build()
            .expect("fail to create tokio runtime");
        let mut handles = Vec::new();
        for (_, v) in cmds.iter() {
            let cmd = String::clone(&v.cmd);
            let args = Option::clone(&v.args);
            let mut real_cmd = Command::new(cmd);
            if args.is_some() {
                real_cmd.arg(args.unwrap());
            }
            handles.push(rt.spawn(async move {
                real_cmd.output().unwrap();
            }));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::executor::{ExecOptions, Executor};

    use super::WindowsExecutor;

    #[test]
    fn test() {
        let op = ExecOptions::from_cfg("cfg_path");
        let we = WindowsExecutor::new(Arc::new(op));
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_io()
            .thread_name("tokio-")
            .build()
            .expect("fail to create tokio runtime");

        rt.block_on(async move {
            we.exec().await.expect("fail to exec code");
        });
    }
}
