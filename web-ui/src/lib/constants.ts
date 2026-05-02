export const VERSION = "0.1.0";
export const GITHUB_REPO_URL = "https://github.com/dthung1602/ferrisvisor";

export const PROCESS_STATES = [
    "STOPPED",
    "STARTING",
    "RUNNING",
    "BACKOFF",
    "STOPPING",
    "EXITED",
    "FATAL",
    "UNKNOWN"
] as const;

export type ProcessState = (typeof PROCESS_STATES)[number];
