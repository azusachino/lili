mod default_executor;

use crate::Result;
use default_executor::DefaultExecutor;
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
    pub debug_output: Option<bool>,
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

pub trait Executor {
    fn exec(&self) -> Result<()>;
}

pub fn get_executor(options: Arc<ExecOptions>) -> Box<dyn Executor + Send + 'static> {
    Box::new(DefaultExecutor::new(options))
}

#[cfg(test)]
mod tests {
    use std::{
        fs::OpenOptions,
        io::{BufReader, Read},
    };

    use crate::LILI_DEFAULT_CFG;

    use super::Executor;
    use anyhow::Result;

    struct TestExecutor {
        message: String,
    }

    impl Executor for TestExecutor {
        fn exec(&self) -> Result<()> {
            println!("{}", self.message);
            Ok(())
        }
    }

    #[test]
    fn test() {
        let executor = TestExecutor {
            message: "hello from tokio".to_owned(),
        };
        executor.exec().expect("fail to exec code");
    }

    #[test]
    fn cfg() {
        let path = LILI_DEFAULT_CFG.as_str();
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
