import type { ProcessState } from "$lib/constants";

export interface ProcessInfo {
  name: string;
  group: string;
  description: string;
  start: number;
  stop: number;
  now: number;
  state: number;
  statename: ProcessState;
  spawnerr: string;
  exitstatus: number;
  logfile: string;
  stdout_logfile: string;
  stderr_logfile: string;
  pid: number;
}

export interface ProcessConfig {
  name: string;
  group: string;
  command: string;
  directory: string;
  autostart: boolean;
  startsecs: number;
  startretries: number;
  stopsignal: number;
  stopwaitsecs: number;
  exitcodes: number[];
  redirect_stderr: boolean;
  stdout_logfile: string;
  stdout_logfile_maxbytes: number;
  stdout_logfile_backups: number;
  stdout_capture_maxbytes: number;
  stdout_events_enabled: boolean;
  stdout_syslog: boolean;
  stderr_logfile: string;
  stderr_logfile_maxbytes: number;
  stderr_logfile_backups: number;
  stderr_capture_maxbytes: number;
  stderr_events_enabled: boolean;
  stderr_syslog: boolean;
  process_prio: number;
  group_prio: number;
  uid: string;
  serverurl: string;
  killasgroup: boolean;
  inuse: boolean;
}

export interface ProcessResponse {
  group_id: number;
  host_id: number;
  process: ProcessInfo;
}

export interface ProcessConfigResponse {
  host_id: number;
  config: ProcessConfig;
}

export interface ProcessActionResponse {
  host_id: number;
  process_name: string;
  success: boolean;
  error: string | null;
}

export interface ProcessConfigRequest {
  host_id: number;
  process_name?: string | null;
}

export interface ProcessActionRequest {
  host_id: number;
  process_name: string;
}

export type ProcessAction = "start" | "stop" | "restart";

export async function list(
  group_id: number | null,
  host_id: number | null,
  process_name: string | null
): Promise<ProcessResponse[]> {
  const search = new URLSearchParams();
  if (group_id !== null) {
    search.append("group_id", group_id.toString());
  }
  if (host_id !== null) {
    search.append("host_id", host_id.toString());
  }
  if (process_name !== null) {
    search.append("process_name", process_name);
  }

  const resp = await fetch("/api/process?" + search.toString());

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as ProcessResponse[];
}

export async function action(action: ProcessAction, reqs: ProcessActionRequest[]): Promise<ProcessActionResponse[]> {
  const resp = await fetch(`/api/process/${action}`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(reqs),
  });

  if (!resp.ok) {
    const message = resp.status + " " + resp.statusText;
    console.error("Got response " + message, await resp.text());
    throw new Error(message);
  }

  return (await resp.json()) as ProcessActionResponse[];
}


export default { list, action };
