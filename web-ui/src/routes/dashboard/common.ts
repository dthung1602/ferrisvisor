export type ColumnId = "process" | "state" | "pid" | "last_changed" | "actions";

export type ProcessColumn = {
  id: ColumnId;
  label: string;
  visible: boolean;
  locked: boolean;
};

export const STATE_COLOR_MAP = {
  STOPPED: {
    bg: "bg-tertiary-500/10",
    text: "text-tertiary-500"
  },
  STARTING: {
    bg: "bg-success-500/10",
    text: "text-success-500"
  },
  RUNNING: {
    bg: "bg-success-500/10",
    text: "text-success-500"
  },
  BACKOFF: {
    bg: "bg-warning-500/10",
    text: "text-warning-500"
  },
  STOPPING: {
    bg: "bg-warning-500/10",
    text: "text-warning-500"
  },
  EXITED: {
    bg: "bg-tertiary-500/10",
    text: "text-tertiary-500"
  },
  FATAL: {
    bg: "bg-error-500/10",
    text: "text-error-500"
  },
  UNKNOWN: {
    bg: "bg-surface-500/10",
    text: "text-surface-500"
  }
};

export const STATE_ACTION_MAP = {
  STOPPED: "start",
  STARTING: "stop",
  RUNNING: "stop",
  BACKOFF: "start",
  STOPPING: "stop",
  EXITED: "start",
  FATAL: "start",
  UNKNOWN: "start"
};