use crate::supervisor::{ProcessConfig, ProcessInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessQuery {
    pub group_id: Option<i32>,
    pub host_id: Option<i32>,
    pub process_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DisplayProcess {
    pub group_id: i32,
    pub host_id: i32,
    pub process: ProcessInfo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessConfigQuery {
    pub host_id: i32,
    pub process_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DisplayProcessConfig {
    pub host_id: i32,
    pub config: ProcessConfig,
}

#[derive(Debug, Deserialize)]
pub struct ProcessActionRequest {
    pub host_id: i32,
    pub process_name: String,
}

#[derive(Debug, Serialize)]
pub struct ProcessActionResult {
    pub host_id: i32,
    pub process_name: String,
    pub success: bool,
    pub error: Option<String>,
}
