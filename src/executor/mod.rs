#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use crate::{async_trait, Result};

pub struct ExecOptions {
    pub cmd: String,
    pub args: Option<String>,
    pub ignore_output: bool,
}

impl ExecOptions {
    pub fn new(cmd: &str) -> Self {
        Self {
            cmd: String::from(cmd),
            args: None,
            ignore_output: true,
        }
    }

    pub fn args(&mut self, args: &str) {
        self.args = Some(String::from(args));
    }

    pub fn ignore_output(&mut self, ignore: bool) {
        self.ignore_output = ignore;
    }
}

#[async_trait]
pub trait Executor {
    // executor
    async fn exec(&self) -> Result<()>;
}

pub fn get_executor(options: ExecOptions) -> Box<dyn Executor + Send + Sync> {
    #[cfg(target_os = "windows")]
    return WindowExecutor::new(options);
    #[cfg(target_os = "linux")]
    get_linux_executor(options)
}

#[cfg(target_os = "windows")]
fn get_windows_executor(options: ExecOptions) -> Box<dyn Executor> {
    use self::windows::WindowsExecutor;
    let we = WindowsExecutor::new(options);
    Box::new(we)
}

#[cfg(target_os = "linux")]
fn get_linux_executor(options: ExecOptions) -> Box<dyn Executor + Send + Sync> {
    use self::linux::LinuxExecutor;
    let le = LinuxExecutor::new(options);
    Box::new(le)
}
