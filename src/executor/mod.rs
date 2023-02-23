#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use crate::{async_trait, Result};
use serde::Deserialize;
use std::{collections::HashMap, fs::OpenOptions, io::BufReader};

#[derive(Deserialize, Debug)]
pub struct Process {
    pub cmd: String,
    pub args: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ExecOptions {
    pub processes: HashMap<String, Process>,
    pub ignore_output: Option<bool>,
}

impl ExecOptions {
    pub fn from_cfg(cfg_path: &str) -> Self {
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

pub fn get_executor(options: ExecOptions) -> Box<dyn Executor> {
    #[cfg(target_os = "windows")]
    return get_windows_executor(options);
    #[cfg(target_os = "linux")]
    return get_linux_executor(options);
}

#[cfg(target_os = "windows")]
fn get_windows_executor(options: ExecOptions) -> Box<dyn Executor> {
    use self::windows::WindowsExecutor;
    Box::new(WindowsExecutor::new(options))
}

#[cfg(target_os = "linux")]
fn get_linux_executor(options: ExecOptions) -> Box<dyn Executor> {
    use self::linux::LinuxExecutor;
    Box::new(LinuxExecutor::new(options))
}
