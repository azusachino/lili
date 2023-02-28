use crate::LILI_DIR;

use super::{ExecOptions, Executor};
use std::{
    fs::OpenOptions,
    process::{Command, Stdio},
    sync::Arc,
};

pub struct LinuxExecutor {
    options: Arc<ExecOptions>,
    log_dir: String,
}

impl LinuxExecutor {
    pub fn new(options: Arc<ExecOptions>) -> Self {
        let dir = shellexpand::tilde(
            format!("{}{}{}", LILI_DIR, std::path::MAIN_SEPARATOR, "debug_log").as_str(),
        )
        .to_string();
        Self {
            options,
            log_dir: dir,
        }
    }
}

impl Executor for LinuxExecutor {
    fn exec(&self) -> anyhow::Result<()> {
        let cmds = self.options.processes.lock().expect("fail to read options");

        let mut handles = Vec::new();
        for (name, v) in cmds.iter() {
            let cmd = String::clone(&v.cmd);
            let args = Option::clone(&v.args);
            let log_file = OpenOptions::new()
                .create(true)
                .truncate(true)
                .append(true)
                .open(format!(
                    "{}{}{name}.log",
                    self.log_dir,
                    std::path::MAIN_SEPARATOR
                ))
                .expect(&format!("fail to open {name}.log"));
            let mut real_cmd = Command::new(cmd);
            if args.is_some() {
                real_cmd.arg(args.unwrap());
            }
            real_cmd.stdout(Stdio::from(log_file));
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
