use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::BufReader,
    path::MAIN_SEPARATOR,
    process::{Command, Stdio},
    sync::Mutex,
};

use crate::{Result, LILI_DIR};

#[derive(Deserialize, Debug)]
pub struct Process {
    pub cmd: String,
    pub args: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ExecOptions {
    pub processes: Mutex<HashMap<String, Process>>,
    pub debug_output: Option<bool>,
}

pub struct Executor {
    options: ExecOptions,
    log_dir: String,
}

impl ExecOptions {
    /**
     * load exec options from cfg
     *
     * @param cfg_path should be absolute path
     */
    pub fn from_cfg(cfg_path: &str) -> Result<Self> {
        let f = OpenOptions::new()
            .read(true)
            .write(false)
            .append(false)
            .open(cfg_path)?;

        let buf_r = BufReader::new(&f);
        Ok(serde_yaml::from_reader(buf_r)?)
    }
}

impl Executor {
    pub fn new(options: ExecOptions) -> Self {
        let path = format!("{}{}debug", LILI_DIR.as_str(), MAIN_SEPARATOR);
        let dir = shellexpand::tilde(path.as_str()).to_string();
        Self {
            options,
            log_dir: dir,
        }
    }

    pub async fn exec(&self) -> Result<()> {
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
    use std::{
        fs::OpenOptions,
        io::{BufReader, Read},
    };

    use crate::LILI_DEFAULT_EXEC_CFG;

    #[test]
    fn cfg() {
        let path = LILI_DEFAULT_EXEC_CFG.as_str();
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
