// local mod
pub mod executor;
pub mod generator;

// extern library mod
pub use anyhow::Result;
pub use std::path::MAIN_SEPARATOR;

#[macro_use]
extern crate lazy_static;

// global constants
lazy_static! {
    // default config dir
    pub static ref LILI_DIR: String = format!("~{}.lili", MAIN_SEPARATOR);
    // default config.yaml
    pub static ref LILI_DEFAULT_CFG: String = format!(
        "~{}.lili{}config.yaml", MAIN_SEPARATOR, MAIN_SEPARATOR
    );
}
