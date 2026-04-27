<script lang="ts">
  import { ChevronDown, Columns2, Logs, Play, RotateCcw, Server, Square } from "@lucide/svelte";
  import { formatDistanceToNowStrict } from "date-fns";

  import type { Host } from "$lib/api/host";
  import type { ProcessInfo } from "$lib/api/process";
  import { PROCESS_STATES, type ProcessState } from "$lib/common";

  import type { ProcessColumn } from "./common.ts";

  type Props = {
    columnConfigOpen: boolean;
    columns: ProcessColumn[];
    hosts: Host[];
    processInfoByHost: Map<number, ProcessInfo[]>;
    selectedHostId: number | null;
    serviceRegex: string | null;
    selectedProcessState: ProcessState | null;
  };

  let {
    columnConfigOpen = $bindable(false),
    columns,
    hosts,
    processInfoByHost,
    selectedHostId,
    serviceRegex,
    selectedProcessState
  }: Props = $props();

  let collapsedHosts = $state<Record<number, boolean>>({});

  function toggleHost(id: number) {
    collapsedHosts[id] = !collapsedHosts[id];
  }

  function getFilterProcesses(processes: ProcessInfo[]) {
    return processes.filter((p) => {
      if (serviceRegex && !p.name.match(serviceRegex)) {
        return false;
      }
      if (selectedProcessState && p.statename !== selectedProcessState) {
        return false;
      }
      return true;
    });
  }

  function summarizeHostStats(processes: ProcessInfo[]): Record<ProcessState, number> {
    const stats = Object.fromEntries(PROCESS_STATES.map((state) => [state, 0])) as Record<ProcessState, number>;
    for (let process of processes) {
      stats[process.statename]++;
    }
    return stats;
  }

  function getProcessLastChange(process: ProcessInfo): string {
    const timestampSec = process.stop || process.start;
    if (timestampSec === 0) {
      return "N/A";
    }
    return formatDistanceToNowStrict(new Date(timestampSec * 1000));
  }

  const STATE_COLOR_MAP = {
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

  const STATE_ACTION_MAP = {
    STOPPED: "start",
    STARTING: "stop",
    RUNNING: "stop",
    BACKOFF: "start",
    STOPPING: "stop",
    EXITED: "start",
    FATAL: "stop",
    UNKNOWN: "start"
  };
</script>

<div class="space-y-6">
  {#each hosts as host (host.id)}
    {@const isCollapsed = !!collapsedHosts[host.id]}
    {@const hostProcesses = processInfoByHost.get(host.id) ?? []}
    {@const hostStats = summarizeHostStats(hostProcesses)}
    {@const filteredProcesses = getFilterProcesses(hostProcesses)}

    {#if (!selectedHostId || selectedHostId === host.id) && (!selectedProcessState || hostStats[selectedProcessState] > 0) && filteredProcesses.length > 0}
      <div class="overflow-hidden rounded-xl border border-surface-500/50 bg-surface-200-800 shadow-2xl">
        <!-- Host Header (Clickable Toggle) -->
        <!-- TODO handle offline hosts-->
        <button
          onclick={() => toggleHost(host.id)}
          class="flex w-full items-center justify-between border-l-4 border-secondary-500 p-4 text-left transition-colors hover:bg-surface-500/20"
        >
          <div class="flex flex-wrap items-center gap-6">
            <div class="flex items-center gap-4">
              <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-surface-500/30">
                <Server class="text-secondary-500" size="24" />
              </div>
              <div>
                <h3 class="text-lg font-bold text-surface-900-100">{host.name}</h3>
                <p class="font-mono text-xs font-medium text-surface-900-100">{host.hostname}:{host.port}</p>
              </div>
            </div>

            <div class="flex gap-2">
              {#each PROCESS_STATES as state (state)}
                {@const count = hostStats[state]}
                {@const { bg, text } = STATE_COLOR_MAP[state]}
                {#if count > 0}
                  <span
                    class="{bg} {text} flex items-center rounded-md px-2.5 py-1 text-xs font-bold tracking-widest uppercase"
                  >
                    {count}
                    {state}
                  </span>
                {/if}
              {/each}
            </div>
          </div>

          <ChevronDown
            class="size-5 text-surface-200 transition-transform duration-300 {isCollapsed ? '' : 'rotate-180'}"
          />
        </button>

        <!-- Processes Table (Collapsible) -->
        {#if !isCollapsed && filteredProcesses.length > 0}
          <div class="overflow-x-auto border-t border-surface-500/50 bg-surface-100-900/60 p-4">
            <table class="table w-full border-collapse text-left">
              <!--  Table Header  -->
              <thead>
                <tr
                  class="border-b border-surface-500/50 bg-surface-300-700/50 text-xs font-black tracking-widest text-surface-900-100 uppercase"
                >
                  {#each columns as column (column.label)}
                    {#if column.visible}
                      {#if column.id === "actions"}
                        <th class="px-4 py-3 text-right">
                          <div class="flex items-center justify-end gap-2">
                            {column.label}
                            <button
                              onclick={() => (columnConfigOpen = true)}
                              class="rounded p-1 text-surface-500 transition-colors hover:bg-surface-500/10 hover:text-primary-500"
                            >
                              <Columns2 size="14" />
                            </button>
                          </div>
                        </th>
                      {:else}
                        <th class="px-4 py-3">{column.label}</th>
                      {/if}
                    {/if}
                  {/each}
                </tr>
              </thead>

              <!--  Table Body  -->
              <tbody>
                {#each filteredProcesses as process (process.name)}
                  {@const isFatal = process.statename === "FATAL"}
                  {@const { text: stateTxt } = STATE_COLOR_MAP[process.statename]}
                  {@const action = STATE_ACTION_MAP[process.statename]}

                  <tr class="group border-b border-surface-500/5 transition-colors hover:bg-surface-500/5">
                    {#each columns as column (column.id)}
                      {#if column.visible}
                        <!-- Process Name -->
                        {#if column.id === "process"}
                          <td class="px-4 py-3 font-medium {isFatal ? 'text-error-500' : 'text-surface-900-100'}">
                            {process.name}
                          </td>

                          <!-- Process State -->
                        {:else if column.id === "state"}
                          <td class="px-4 py-3">
                            <span class="flex items-center gap-2 font-bold {stateTxt}">
                              {process.statename}
                            </span>
                          </td>

                          <!-- PID -->
                        {:else if column.id === "pid"}
                          <td class="px-4 py-3 font-mono font-medium text-surface-900-100">
                            {process.pid ?? "--"}
                          </td>

                          <!-- Last changed -->
                        {:else if column.id === "last_changed"}
                          <td class="px-4 py-3 font-medium text-surface-900-100">
                            {getProcessLastChange(process)}
                          </td>

                          <!-- Actions -->
                        {:else if column.id === "actions"}
                          <td class="px-4 py-3 text-right">
                            <div
                              class="flex items-center justify-end gap-1 opacity-70 transition-opacity group-hover:opacity-100"
                            >
                              {#if action === "stop"}
                                <button
                                  class="flex items-center justify-center gap-1.5 rounded-md p-1.5 px-2 text-error-500 transition-colors hover:bg-error-500/20"
                                >
                                  <Square size="16" /> Stop
                                </button>
                              {:else if action === "start"}
                                <button
                                  class="flex items-center justify-center gap-1.5 rounded-md p-1.5 px-2 text-success-500 transition-colors hover:bg-success-500/20"
                                >
                                  <Play size="16" /> Start
                                </button>
                              {/if}

                              <button
                                class="flex items-center justify-center gap-1.5 rounded-md p-1.5 px-2 text-primary-500 transition-colors hover:bg-primary-500/20"
                              >
                                <RotateCcw size="16" /> Restart
                              </button>
                              <button
                                class="flex items-center justify-center gap-1.5 rounded-md p-1.5 px-2 text-surface-900-100 transition-colors hover:bg-surface-500/50"
                              >
                                <Logs size="16" /> Info
                              </button>
                            </div>
                          </td>
                        {/if}
                      {/if}
                    {/each}
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>
    {/if}
  {/each}
</div>
