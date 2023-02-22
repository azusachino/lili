#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use crate::Result;

pub use windows::WindowsExecutor;

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

pub trait Executor {
    // buidler
    fn new(options: ExecOptions) -> Self;
    // executor
    fn exec(&self) -> Result<()>;
}
