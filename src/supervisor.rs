use dxr;
use dxr::TryFromValue;
use dxr_client::Error;
use dxr_client::Url;
use dxr_client::{Client as DxrClient, ClientBuilder as DxrClientBuilder};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(TryFromValue, Debug)]
pub struct SupervisorState {
    pub statecode: i32,
    pub statename: String,
}

#[derive(TryFromValue, Debug)]
pub struct ProcessInfo {
    pub name: String,
    pub group: String,
    pub description: String,
    pub start: i32,
    pub stop: i32,
    pub now: i32,
    pub state: i32,
    pub statename: String,
    pub spawnerr: String,
    pub exitstatus: i32,
    pub logfile: String,
    pub stdout_logfile: String,
    pub stderr_logfile: String,
    pub pid: i32,
}

#[derive(Debug, TryFromValue)]
pub struct ProcessConfig {
    pub name: String,
    pub group: String,
    pub command: String,
    pub directory: String,
    pub autostart: bool,
    pub startsecs: i32,
    pub startretries: i32,
    pub stopsignal: i32,
    pub stopwaitsecs: i32,
    pub exitcodes: Vec<i32>,
    pub redirect_stderr: bool,
    pub stdout_logfile: String,
    pub stdout_logfile_maxbytes: i32,
    pub stdout_logfile_backups: i32,
    pub stdout_capture_maxbytes: i32,
    pub stdout_events_enabled: bool,
    pub stdout_syslog: bool,
    pub stderr_logfile: String,
    pub stderr_logfile_maxbytes: i32,
    pub stderr_logfile_backups: i32,
    pub stderr_capture_maxbytes: i32,
    pub stderr_events_enabled: bool,
    pub stderr_syslog: bool,
    pub process_prio: i32,
    pub group_prio: i32,
    pub uid: String,
    pub serverurl: String,
    pub killasgroup: bool,
    pub inuse: bool,
}

#[derive(TryFromValue, Debug)]
pub struct ProcessStatus {
    pub name: String,
    pub group: String,
    pub status: i32,
    pub description: String,
}

pub struct Server {
    url: Url,
    rpc_client: DxrClient,
}

impl Server {
    pub fn new(host: &str, port: u16, username: Option<&str>, password: Option<&str>) -> Self {
        let auth_str = if let (Some(u), Some(p)) = (username, password) {
            format!("{}:{}@", u, p)
        } else {
            "".to_string()
        };
        let url = Url::parse(&format!("http://{}{}:{}/RPC2", auth_str, host, port)).unwrap();
        let rpc_client = DxrClientBuilder::new(url.clone()).build();
        Self { url, rpc_client }
    }

    // status & control
    pub async fn get_api_version(&self) -> Result<String> {
        self.rpc_client
            .call("supervisor.getAPIVersion", ())
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_supervisor_version(&self) -> Result<String> {
        self.rpc_client
            .call("supervisor.getSupervisorVersion", ())
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_state(&self) -> Result<SupervisorState> {
        self.rpc_client
            .call("supervisor.getState", ())
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_pid(&self) -> Result<i32> {
        self.rpc_client
            .call("supervisor.getPID", ())
            .await
            .map_err(|e| e.into())
    }

    pub async fn read_log(&self, offset: i32, length: i32) -> Result<String> {
        self.rpc_client
            .call("supervisor.readLog", (offset, length))
            .await
            .map_err(|e| e.into())
    }

    pub async fn clear_log(&self) -> Result<bool> {
        self.rpc_client
            .call("supervisor.clearLog", ())
            .await
            .map_err(|e| e.into())
    }

    pub async fn shutdown(&self) -> Result<bool> {
        self.rpc_client
            .call("supervisor.shutdown", ())
            .await
            .map_err(|e| e.into())
    }

    pub async fn restart(&self) -> Result<bool> {
        self.rpc_client
            .call("supervisor.restart", ())
            .await
            .map_err(|e| e.into())
    }

    // Process Control
    pub async fn get_process_info(&self, name: &str) -> Result<ProcessInfo> {
        self.rpc_client
            .call("supervisor.getProcessInfo", (name,))
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_all_process_info(&self) -> Result<Vec<ProcessInfo>> {
        self.rpc_client
            .call("supervisor.getAllProcessInfo", ())
            .await
            .map_err(|e| e.into())
    }

    pub async fn start_process(&self, name: &str, wait: bool) -> Result<bool> {
        self.rpc_client
            .call("supervisor.startProcess", (name, wait))
            .await
            .map_err(|e| e.into())
    }

    pub async fn start_all_processes(&self, wait: bool) -> Result<Vec<ProcessStatus>> {
        self.rpc_client
            .call("supervisor.startAllProcesses", (wait,))
            .await
            .map_err(|e| e.into())
    }

    pub async fn start_process_group(&self, name: &str, wait: bool) -> Result<Vec<ProcessStatus>> {
        self.rpc_client
            .call("supervisor.startProcessGroup", (name, wait))
            .await
            .map_err(|e| e.into())
    }

    pub async fn stop_process(&self, name: &str, wait: bool) -> Result<bool> {
        self.rpc_client
            .call("supervisor.stopProcess", (name, wait))
            .await
            .map_err(|e| e.into())
    }

    pub async fn stop_all_processes(&self, wait: bool) -> Result<Vec<ProcessStatus>> {
        self.rpc_client
            .call("supervisor.stopAllProcesses", (wait,))
            .await
            .map_err(|e| e.into())
    }

    pub async fn stop_process_group(&self, name: &str, wait: bool) -> Result<Vec<ProcessStatus>> {
        self.rpc_client
            .call("supervisor.stopProcessGroup", (name, wait))
            .await
            .map_err(|e| e.into())
    }

    pub async fn signal_process(&self, name: &str, signal: &str) -> Result<bool> {
        self.rpc_client
            .call("supervisor.signalProcess", (name, signal))
            .await
            .map_err(|e| e.into())
    }

    pub async fn signal_process_group(
        &self,
        name: &str,
        signal: &str,
    ) -> Result<Vec<ProcessStatus>> {
        self.rpc_client
            .call("supervisor.signalProcessGroup", (name, signal))
            .await
            .map_err(|e| e.into())
    }

    pub async fn signal_all_processes(&self, signal: &str) -> Result<Vec<ProcessStatus>> {
        self.rpc_client
            .call("supervisor.signalAllProcesses", (signal,))
            .await
            .map_err(|e| e.into())
    }

    pub async fn send_process_stdin(&self, name: &str, chars: &str) -> Result<bool> {
        self.rpc_client
            .call("supervisor.sendProcessStdin", (name, chars))
            .await
            .map_err(|e| e.into())
    }

    pub async fn send_remote_comm_event(&self, event_type: &str, data: &str) -> Result<bool> {
        self.rpc_client
            .call("supervisor.sendRemoteCommEvent", (event_type, data))
            .await
            .map_err(|e| e.into())
    }

    pub async fn reload_config(&self) -> Result<Vec<Vec<Vec<String>>>> {
        self.rpc_client
            .call("supervisor.reloadConfig", ())
            .await
            .map_err(|e| e.into())
    }

    pub async fn add_process_group(&self, name: &str) -> Result<bool> {
        self.rpc_client
            .call("supervisor.addProcessGroup", (name,))
            .await
            .map_err(|e| e.into())
    }

    pub async fn remove_process_group(&self, name: &str) -> Result<bool> {
        self.rpc_client
            .call("supervisor.removeProcessGroup", (name,))
            .await
            .map_err(|e| e.into())
    }

    pub async fn get_all_config_info(&self) -> Result<Vec<ProcessConfig>> {
        self.rpc_client
            .call("supervisor.getAllConfigInfo", ())
            .await
            .map_err(|e| e.into())
    }

    // Process Logs
    pub async fn read_process_stdout_log(
        &self,
        name: &str,
        offset: i32,
        length: i32,
    ) -> Result<String> {
        self.rpc_client
            .call("supervisor.readProcessStdoutLog", (name, offset, length))
            .await
            .map_err(|e| e.into())
    }

    pub async fn read_process_stderr_log(
        &self,
        name: &str,
        offset: i32,
        length: i32,
    ) -> Result<String> {
        self.rpc_client
            .call("supervisor.readProcessStderrLog", (name, offset, length))
            .await
            .map_err(|e| e.into())
    }

    pub async fn tail_process_stdout_log(
        &self,
        name: &str,
        offset: i64,
        length: i32,
    ) -> Result<(String, i32, bool)> {
        self.rpc_client
            .call("supervisor.tailProcessStdoutLog", (name, offset, length))
            .await
            .map_err(|e| e.into())
    }

    pub async fn tail_process_stderr_log(
        &self,
        name: &str,
        offset: i64,
        length: i32,
    ) -> Result<(String, i32, bool)> {
        self.rpc_client
            .call("supervisor.tailProcessStderrLog", (name, offset, length))
            .await
            .map_err(|e| e.into())
    }

    pub async fn clear_process_logs(&self, name: &str) -> Result<bool> {
        self.rpc_client
            .call("supervisor.clearProcessLogs", (name,))
            .await
            .map_err(|e| e.into())
    }

    pub async fn clear_all_process_logs(&self) -> Result<Vec<ProcessStatus>> {
        self.rpc_client
            .call("supervisor.clearAllProcessLogs", ())
            .await
            .map_err(|e| e.into())
    }
}
