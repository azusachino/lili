pub mod executor;

pub use anyhow::Result;
pub use std::path::MAIN_SEPARATOR;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref LILI_DIR: String = format!("~{}.lili", MAIN_SEPARATOR);
    pub static ref LILI_DEFAULT_EXEC_CFG: String = format!(
        "~{}.lili{}executor{}cfg.yaml",
        MAIN_SEPARATOR, MAIN_SEPARATOR, MAIN_SEPARATOR
    );
}
