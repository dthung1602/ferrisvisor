export const PROCESS_STATES = [
  "STOPPED",
  "STARTING",
  "RUNNING",
  "BACKOFF",
  "STOPPING",
  "EXITED",
  "FATAL",
  "UNKNOWN",
] as const;

export type ProcessState = typeof PROCESS_STATES[number];

