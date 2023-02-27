#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use crate::{async_trait, Result};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::BufReader,
    sync::{Arc, Mutex},
};

#[derive(Deserialize, Debug)]
pub struct Process {
    pub cmd: String,
    pub args: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ExecOptions {
    pub processes: Mutex<HashMap<String, Process>>,
    pub ignore_output: Option<bool>,
}

impl ExecOptions {
    pub fn from_cfg(cfg_path: &str) -> Self {
        let cfg_path = shellexpand::tilde(cfg_path).to_string();
        let f = OpenOptions::new()
            .read(true)
            .write(false)
            .append(false)
            .open(cfg_path)
            .expect("fail to open config file");

        let buf_r = BufReader::new(&f);

        serde_yaml::from_reader(buf_r).expect("fail to parse yaml")
    }
}

#[async_trait]
pub trait Executor {
    // executor
    async fn exec(&self) -> Result<()>;
}

pub fn get_executor(options: Arc<ExecOptions>) -> Box<dyn Executor + Send + 'static> {
    #[cfg(target_os = "windows")]
    {
        use windows::WindowsExecutor;
        Box::new(WindowsExecutor::new(options))
    }
    #[cfg(target_os = "linux")]
    {
        use linux::LinuxExecutor;
        Box::new(LinuxExecutor::new(options))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::OpenOptions,
        io::{BufReader, Read},
    };

    use super::Executor;
    use anyhow::Result;
    use async_trait::async_trait;

    struct TestExecutor {
        message: String,
    }

    #[async_trait]
    impl Executor for TestExecutor {
        async fn exec(&self) -> Result<()> {
            println!("{}", self.message);
            Ok(())
        }
    }

    #[test]
    fn test() {
        let executor = TestExecutor {
            message: "hello from tokio".to_owned(),
        };

        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_io()
            .thread_name("tokio-")
            .build()
            .expect("fail to create tokio runtime");

        rt.block_on(async move {
            executor.exec().await.expect("fail to exec code");
        });
    }

    #[test]
    fn cfg() {
        let path = "~/.lili/cfg.yaml";
        let real_path = shellexpand::tilde(path);
        let f = OpenOptions::new()
            .read(true)
            .write(false)
            .append(false)
            .open(real_path.to_string())
            .expect("fail to open config file");

        let mut buf = String::new();
        let mut buf_r = BufReader::new(&f);
        buf_r.read_to_string(&mut buf).expect("fail to read");
        println!("{}", buf);
    }
}
