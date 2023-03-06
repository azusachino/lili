pub mod cmds;

pub use anyhow::Result;
pub use std::path::MAIN_SEPARATOR;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref LILI_DIR: String =
        shellexpand::tilde(&format!("~{}.lili", MAIN_SEPARATOR)).to_string();
    pub static ref LILI_DEFAULT_EXEC_CFG: String = format!(
        "{}{}executor{}cfg.yaml",
        LILI_DIR.as_str(),
        MAIN_SEPARATOR,
        MAIN_SEPARATOR
    );
}
