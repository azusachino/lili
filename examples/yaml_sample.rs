use std::{collections::HashMap, fs::OpenOptions, io::BufReader};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Process {
    pub cmd: String,
    pub args: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ExecOptions {
    pub processes: HashMap<String, Process>,
    pub ignore_output: bool,
}

fn main() {
    let f = OpenOptions::new()
        .read(true)
        .write(false)
        .append(false)
        .open("./default_cfg.yaml")
        .expect("fail to open file");

    let buf_r = BufReader::new(&f);

    let r: ExecOptions = serde_yaml::from_reader(buf_r).expect("fail to parse yaml");

    println!("{:?}", r);
}
