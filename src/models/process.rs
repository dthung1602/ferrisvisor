use crate::supervisor::{ProcessConfig, ProcessInfo};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessRequest {
    pub group_id: Option<i32>,
    pub host_id: Option<i32>,
    pub process_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProcessResponse {
    pub group_id: i32,
    pub host_id: i32,
    pub process: ProcessInfo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessConfigRequest {
    pub host_id: i32,
    pub process_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ProcessConfigResponse {
    pub host_id: i32,
    pub config: ProcessConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProcessActionRequest {
    pub host_id: i32,
    pub process_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProcessActionResponse {
    pub host_id: i32,
    pub process_name: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProcessLogRequest {
    pub host_id: i32,
    pub process_name: String,
    pub offset: i64,
    pub length: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct ProcessLogResponse {
    pub log: String,
    pub offset: i32,
    pub overflow: bool,
}
