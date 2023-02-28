use std::{
    fs::OpenOptions,
    path::MAIN_SEPARATOR,
    process::{Command, Stdio},
    sync::Arc,
};

use crate::LILI_DIR;

use super::{ExecOptions, Executor};

pub struct WindowsExecutor {
    options: Arc<ExecOptions>,
    log_dir: String,
}

impl WindowsExecutor {
    pub fn new(options: Arc<ExecOptions>) -> Self {
        let path = format!("{}{}debug", LILI_DIR.as_str(), MAIN_SEPARATOR);
        let dir = shellexpand::tilde(path.as_str()).to_string();
        Self {
            options,
            log_dir: dir,
        }
    }
}

impl Executor for WindowsExecutor {
    fn exec(&self) -> anyhow::Result<()> {
        let cmds = self.options.processes.lock().expect("fail to read options");

        let mut handles = Vec::new();
        for (name, v) in cmds.iter() {
            let cmd = shellexpand::tilde(&v.cmd).to_string();
            let args = Option::clone(&v.args);
            let mut real_cmd = Command::new(cmd);

            if args.is_some() {
                let arg = shellexpand::tilde(args.unwrap().as_str()).to_string();
                real_cmd.arg(arg);
            }

            if self.options.debug_output.is_some() && self.options.debug_output.unwrap() {
                let final_path = format!("{}{}{}.log", self.log_dir, MAIN_SEPARATOR, name);
                let log_file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(final_path)
                    .expect(&format!("fail to open {name}.log"));
                real_cmd.stdout(Stdio::from(log_file));
            }

            let child = real_cmd.spawn().expect(&format!("fail to start {name}"));
            handles.push(child);
        }

        // Wait for all the child processes to complete
        for child in handles.iter_mut() {
            child.wait().expect("Failed to wait for child process");
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
        we.exec().expect("fail to exec");
    }
}
