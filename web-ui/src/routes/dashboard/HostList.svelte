<script lang="ts">
  import { ChevronDown, Columns2, Play, RotateCcw, Server, Square } from "@lucide/svelte";
  import { Accordion } from "@skeletonlabs/skeleton-svelte";
  import { api } from "$lib";
  import { formatDistanceToNowStrict } from "date-fns";
  import { SvelteMap } from "svelte/reactivity";
  import { slide } from "svelte/transition";

  import type { Host } from "$lib/api/host";
  import type { ProcessAction, ProcessActionRequest, ProcessInfo } from "$lib/api/process";
  import { fullProcessName, wait } from "$lib/common";
  import { PROCESS_STATES, type ProcessState } from "$lib/constants";
  import { toaster } from "$lib/toaster";

  import { STATE_ACTION_MAP, STATE_COLOR_MAP, type ProcessColumn } from "./common.ts";
  import ProcessPopup from "./ProcessPopup.svelte";

  type Props = {
    columnConfigOpen: boolean;
    columns: ProcessColumn[];
    hosts: Host[];
    processInfoByHost: Map<number, ProcessInfo[]>;
    selectedHostId: number | null;
    serviceRegex: string | null;
    selectedProcessState: ProcessState | null;
    refreshAllProcessInfo: () => void;
  };

  let {
    columnConfigOpen = $bindable(false),
    columns,
    hosts,
    processInfoByHost,
    selectedHostId,
    serviceRegex,
    selectedProcessState,
    refreshAllProcessInfo
  }: Props = $props();

  let processInActionMap = new SvelteMap<string, boolean>();

  function processHostKey(hostId: number, proFullName: string) {
    return `${hostId}:${proFullName}`;
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
    const timestampSec = process.stop && process.stop > process.start ? process.stop : process.start;
    if (timestampSec === 0) {
      return "N/A";
    }
    return formatDistanceToNowStrict(new Date(timestampSec * 1000));
  }

  async function handleProcessAction(action: ProcessAction, reqs: ProcessActionRequest[]) {
    try {
      for (let req of reqs) {
        processInActionMap.set(processHostKey(req.host_id, req.process_name), true);
      }

      const respPromise = api.process.action(action, reqs);
      let actionResponseReceived = false;

      wait(200).then(() => {
        if (!actionResponseReceived) {
          refreshAllProcessInfo();
        }
      });

      const resp = await respPromise;
      actionResponseReceived = true;

      const errors = resp
        .filter((r) => !r.success)
        .map((r) => `${r.host_id}:${r.process_name} - ${r.error}`)
        .join("\n");
      if (errors) {
        toaster.error({
          title: `Action failed: ${action}`,
          description: errors
        });
      } else {
        toaster.success({
          title: "Action successful",
          description: `All processes ${action}ed successfully`
        });
      }
      refreshAllProcessInfo();
    } catch (e) {
      console.error("Failed to start processes", e);
      toaster.error({
        title: "System Error",
        description: "An unexpected error occurred while processing the action."
      });
    } finally {
      for (let req of reqs) {
        processInActionMap.set(processHostKey(req.host_id, req.process_name), false);
      }
    }
  }

  let filteredHosts = $derived.by(() => {
    return hosts.filter((host) => {
      const hostProcesses = processInfoByHost.get(host.id) ?? [];
      const hostStats = summarizeHostStats(hostProcesses);
      const filteredProcesses = getFilterProcesses(hostProcesses);

      return (
        (!selectedHostId || selectedHostId === host.id) &&
        (!selectedProcessState || hostStats[selectedProcessState] > 0) &&
        filteredProcesses.length > 0
      );
    });
  });

  // Initialize all visible hosts as open
  let openHostIds = $derived(filteredHosts.map((h) => h.id.toString()));
</script>

<div class="space-y-6">
  <Accordion value={openHostIds} onValueChange={(e) => (openHostIds = e.value)} multiple collapsible class="space-y-4">
    {#each filteredHosts as host (host.id)}
      {@const hostProcesses = processInfoByHost.get(host.id) ?? []}
      {@const hostStats = summarizeHostStats(hostProcesses)}
      {@const filteredProcesses = getFilterProcesses(hostProcesses)}
      {@const isOpen = openHostIds.includes(host.id.toString())}

      <Accordion.Item
        value={host.id.toString()}
        class="gap-0 overflow-hidden rounded-xl border border-surface-500/50 bg-surface-200-800 shadow-2xl"
      >
        <Accordion.ItemTrigger
          class="flex w-full items-center justify-between border-l-4 border-secondary-500 p-4 text-left transition-colors hover:bg-surface-500/20
                 {isOpen ? 'rounded-bl-none' : ''}"
        >
          <div class="flex flex-wrap items-center gap-6">
            <div class="flex items-center gap-4">
              <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-surface-500/30">
                <Server class="text-secondary-700-300" size="24" />
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

          <ChevronDown class="size-5 text-surface-200 transition-transform duration-300 {isOpen ? 'rotate-180' : ''}" />
        </Accordion.ItemTrigger>

        <Accordion.ItemContent>
          {#snippet element(attributes)}
            {#if !attributes.hidden}
              <div
                {...attributes}
                transition:slide={{ duration: 150 }}
                class="overflow-x-auto border-t border-surface-500/50 bg-surface-100-900/60 p-4"
              >
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
                      {@const btnCls =
                        "flex items-center justify-center gap-1.5 rounded-md p-1.5 px-2 transition-colors disabled:text-surface-300 disabled:cursor-not-allowed"}
                      {@const isFatal = process.statename === "FATAL"}
                      {@const { text: stateTxt } = STATE_COLOR_MAP[process.statename]}
                      {@const action = STATE_ACTION_MAP[process.statename]}
                      {@const proFullName = fullProcessName(process)}
                      {@const isProcessInAction = processInActionMap.get(processHostKey(host.id, proFullName)) ?? false}

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
                              {@const req = [{ host_id: host.id, process_name: proFullName }]}

                              <td class="px-4 py-3 text-right">
                                <div
                                  class="flex items-center justify-end gap-1 opacity-70 transition-opacity group-hover:opacity-100"
                                >
                                  {#if action === "stop"}
                                    <button
                                      disabled={isProcessInAction}
                                      class="{btnCls} not-disabled:text-error-500 not-disabled:hover:bg-error-500/20"
                                      onclick={() => handleProcessAction("stop", req)}
                                    >
                                      <Square size="16" /> Stop
                                    </button>
                                  {:else if action === "start"}
                                    <button
                                      disabled={isProcessInAction}
                                      class="{btnCls} not-disabled:text-success-500 not-disabled:hover:bg-success-500/20"
                                      onclick={() => handleProcessAction("start", req)}
                                    >
                                      <Play size="16" /> Start
                                    </button>
                                  {/if}

                                  <button
                                    disabled={isProcessInAction}
                                    class="{btnCls} not-disabled:text-primary-500 not-disabled:hover:bg-primary-500/20"
                                    onclick={() => handleProcessAction("restart", req)}
                                  >
                                    <RotateCcw size="16" /> Restart
                                  </button>
                                  <ProcessPopup {process} {host} />
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
          {/snippet}
        </Accordion.ItemContent>
      </Accordion.Item>
    {/each}
  </Accordion>
</div>
