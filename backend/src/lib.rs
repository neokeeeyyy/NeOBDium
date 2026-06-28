mod cmd;
pub mod dicts;
pub mod mid;
pub mod obd;
mod pid;
mod replay;
mod response;
pub mod scalar;
pub mod vin;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::atomic::AtomicUsize;

pub use cmd::*;
pub use obd::*;
pub use pid::*;
pub use response::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Mode22Pid {
    pub pid: String,
    pub description: String,
    pub equation: String,
    pub unit: String,
    pub value: f32,
    pub supported: bool,
}

pub fn code_desc_db_path() -> PathBuf {
    if let Some(dir) = vin::APP_DATA_DIR.get() {
        dir.join("code-descriptions.sqlite")
    } else {
        PathBuf::from("./data/code-descriptions.sqlite")
    }
}

pub fn mode22_pids_db_path() -> PathBuf {
    if let Some(dir) = vin::APP_DATA_DIR.get() {
        dir.join("model-pids.sqlite")
    } else {
        PathBuf::from("./data/model-pids.sqlite")
    }
}

/// Whether or not to pause OBD threads
/// Keeps track of how many threads want to pause obd
/// If > 0, do not do low priority actions
pub static PAUSE_OBD_COUNT: AtomicUsize = AtomicUsize::new(0);
