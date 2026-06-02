import type { Host } from "$lib/api/host";
import type { ProcessInfo } from "$lib/api/process";
import type { ProcessState } from "$lib/constants";

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

export function filterProcesses(
  processes: ProcessInfo[],
  serviceRegex: string | null,
  selectedProcessState: ProcessState | null
): ProcessInfo[] {
  return processes.filter((p) => {
    if (serviceRegex && !p.name.match(serviceRegex)) {
      return false;
    }
    return !(selectedProcessState && p.statename !== selectedProcessState);
  });
}

export function isHostMatching(
  host: Host,
  hostProcesses: ProcessInfo[],
  selectedHostId: number | null,
  selectedProcessState: ProcessState | null,
  serviceRegex: string | null
): boolean {
  if (selectedHostId && host.id !== selectedHostId) {
    return false;
  }
  const filtered = filterProcesses(hostProcesses, serviceRegex, selectedProcessState);
  return filtered.length > 0;
}
