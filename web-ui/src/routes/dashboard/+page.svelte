<script lang="ts">
  import {
    ChevronDown,
    CircleAlert,
    CirclePlay,
    CircleStop,
    Columns2,
    Play,
    RotateCcw,
    Server,
    Settings,
    Square,
    Terminal,
    X
  } from "@lucide/svelte";
  import { api } from "$lib";

  import type { Group } from "$lib/api/group";
  import type { Host } from "$lib/api/host";
  import type { ProcessInfo, ProcessResponse } from "$lib/api/process";
  import  { PROCESS_STATES, type ProcessState } from "$lib/common";

  import Filters from "./Filters.svelte";
  import GroupSelector from "./GroupSelector.svelte";
  import StatCard from "./StatCard.svelte";
  import { SvelteMap } from "svelte/reactivity";
  import { formatDistanceToNowStrict } from "date-fns";

  type Props = {
    data: { groups: Group[] };
  };
  let { data }: Props = $props();

  let selectedGroupId: number | null = $state(data.groups[0].id ?? null);
  let selectedHostId: number | null = $state(null);
  let serviceRegex = $state("");
  let selectedProcessState: ProcessState | null = $state(null);

  let hosts: Host[] = $state([]);
  let processInfoByHost = $state(new Map<number, ProcessInfo[]>());

  type HostStats = {
    running: number;
    stopped: number;
    fatal: number;
  }

  function summarizeHostStats(processInfos: ProcessInfo[]): HostStats {
    const stats = Object.fromEntries(
      PROCESS_STATES.map(state => [state, 0])
    ) as Record<ProcessState, number>;

    for (let process of processInfos) {
      stats[process.statename]++;
    }

    return {
      running: stats.STARTING + stats.RUNNING + stats.STOPPING,
      fatal: stats.BACKOFF + stats.FATAL + stats.UNKNOWN,
      stopped: stats.STOPPED + stats.EXITED,
    };
  }

  let globalStats: HostStats = $derived.by(() => {
    const hostStats: HostStats[] = [];

    for (let processInfo of processInfoByHost.values()) {
      hostStats.push(summarizeHostStats(processInfo));
    }

    return {
      running: hostStats.reduce((acc, stats) => acc + stats.running, 0),
      fatal: hostStats.reduce((acc, stats) => acc + stats.fatal, 0),
      stopped: hostStats.reduce((acc, stats) => acc + stats.stopped, 0)
    };
  })

  $effect(() => {
    api.process.list(selectedGroupId, selectedHostId, null).then((processResps: ProcessResponse[]) => {
      const newProcessInfoByHost = new SvelteMap<number, ProcessInfo[]>();
      for (const resp of processResps) {
        let infos = newProcessInfoByHost.get(resp.host_id);
        if (infos === undefined) {
          infos = [];
          newProcessInfoByHost.set(resp.host_id, infos);
        }
        infos.push(resp.process);
      }
      processInfoByHost = newProcessInfoByHost
    });
  });

  $effect(() => {
    api.host.list(selectedGroupId).then((data) => {
      hosts = data;
      selectedHostId = null;
      selectedProcessState = null;
    });
  });

  let collapsedHosts = $state<Record<number, boolean>>({});

  $effect(() => {
    console.log({collapsedHosts})
  })

  let columnConfigOpen = $state(false);

  const columns = $state([
    { id: "process", label: "Process", visible: true, locked: true },
    { id: "state", label: "State", visible: true, locked: false },
    { id: "pid", label: "PID", visible: true, locked: false },
    { id: "uptime", label: "Uptime", visible: true, locked: false },
    { id: "actions", label: "Actions", visible: true, locked: true }
  ]);

  function toggleHost(id: number) {
    collapsedHosts[id] = !collapsedHosts[id];
  }

  function getFilterProcesses (processes: ProcessInfo[]) {
    return processes.filter(p => {
      if (serviceRegex && !p.name.match(serviceRegex)) {
        return false;
      }
      if (selectedProcessState && p.statename !== selectedProcessState) {
        return false;
      }
      return true;
    });
  }
</script>

<div class="space-y-8 pb-24">
  <!-- Global Status Dashboard (Bento style) -->
  <div class="relative z-30 grid w-full grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
    <GroupSelector bind:selectedGroupId groups={data.groups} />

    <StatCard title="Running" value={globalStats.running} icon={CirclePlay} colorVariant="success" />

    <StatCard title="Fatal" value={globalStats.fatal} icon={CircleAlert} colorVariant="error" />

    <StatCard title="Exit / Stopped" value={globalStats.stopped} icon={CircleStop} colorVariant="tertiary" />
  </div>

  <!-- Filters & Selectors -->
  <Filters {hosts} bind:serviceRegex bind:selectedProcessState bind:selectedHostId />

  <!-- Host List -->
  <div class="space-y-6">
      {#each hosts as host (host.id)}
        {@const isCollapsed = !!collapsedHosts[host.id]}
        {@const hostProcesses = processInfoByHost.get(host.id) ?? []}
        {@const hostStats = summarizeHostStats(hostProcesses)}
        {@const filteredProcesses = getFilterProcesses(hostProcesses)}

        {#if !selectedHostId || selectedHostId === host.id}
          <div class="overflow-hidden rounded-xl border border-surface-500/50 bg-surface-200-800 shadow-2xl">
          <!-- Host Header (Clickable Toggle) -->
          <!-- TODO handle offline hosts-->
          <button
            onclick={() => toggleHost(host.id)}
            class="flex w-full items-center justify-between border-l-4 p-4 transition-colors hover:bg-surface-500/20 text-left border-secondary-500"
          >
            <div class="flex flex-wrap items-center gap-6">
              <div class="flex items-center gap-4">
                <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-surface-500/30 shrink-0">
                  <Server class='text-secondary-500' size="24" />
                </div>
                <div>
                  <h3 class="text-lg font-bold text-surface-900-100">{host.name}</h3>
                  <p class="font-mono text-xs text-surface-900-100 font-medium">{host.hostname}:{host.port}</p>
                </div>
              </div>

              <div class="flex gap-2">
                {#if hostStats.running > 0}
                  <span class="flex items-center rounded-md bg-secondary-500/10 px-2.5 py-1 text-xs font-bold text-secondary-500 uppercase tracking-widest">
                    <span class="mr-1.5 h-1.5 w-1.5 animate-pulse rounded-full bg-secondary-500"></span>
                    {hostStats.running} Running
                  </span>
                {/if}
                {#if hostStats.fatal > 0}
                  <span class="flex items-center rounded-md bg-error-500/10 px-2.5 py-1 text-xs font-bold text-error-500 uppercase tracking-widest">
                    <span class="mr-1.5 h-1.5 w-1.5 rounded-full bg-error-500"></span>
                    {hostStats.fatal} Fatal
                  </span>
                {/if}
                {#if hostStats.stopped > 0}
                  <span class="flex items-center rounded-md bg-tertiary-500/20 px-2.5 py-1 text-xs font-bold text-tertiary-500 uppercase tracking-widest">
                    {hostStats.stopped} Stopped
                  </span>
                {/if}
              </div>
            </div>

            <ChevronDown class="text-surface-200 size-5 transition-transform duration-300 {isCollapsed ? '' : 'rotate-180'}" />
          </button>

          <!-- Processes Table (Collapsible) -->
          {#if !isCollapsed && filteredProcesses.length > 0}
            <div class="bg-surface-100-900/60 p-4 border-t border-surface-500/50 overflow-x-auto">
              <table class="table w-full text-left border-collapse">
                <thead>
                  <tr class="bg-surface-300-700/50 text-xs text-surface-900-100 font-black uppercase tracking-widest border-b border-surface-500/50">
                    {#if columns.find(c => c.id === 'process')?.visible}<th class="px-4 py-3 font-medium">Process</th>{/if}
                    {#if columns.find(c => c.id === 'state')?.visible}<th class="px-4 py-3 font-medium">State</th>{/if}
                    {#if columns.find(c => c.id === 'pid')?.visible}<th class="px-4 py-3 font-medium">PID</th>{/if}
                    {#if columns.find(c => c.id === 'uptime')?.visible}<th class="px-4 py-3 font-medium">Uptime</th>{/if}
                    <th class="px-4 py-3 font-medium text-right">
                      <div class="flex items-center justify-end gap-2">
                        Actions
                        <button
                          onclick={() => columnConfigOpen = true}
                          class="p-1 hover:bg-surface-500/10 rounded transition-colors text-surface-500 hover:text-primary-500"
                        >
                          <Columns2 size="14" />
                        </button>
                      </div>
                    </th>
                  </tr>
                </thead>
                <tbody>
                  {#each filteredProcesses as process (process.name)}
                    {@const isFatal = process.statename === 'FATAL'}
                    <tr class="border-b border-surface-500/5 hover:bg-surface-500/5 transition-colors group">
                      {#if columns.find(c => c.id === 'process')?.visible}
                        <td class="px-4 py-3 font-medium {isFatal ? 'text-error-500' : 'text-surface-900-100'}">{process.name}</td>
                      {/if}
                      {#if columns.find(c => c.id === 'state')?.visible}
                        <td class="px-4 py-3">
                          <span class="flex items-center gap-2 {process.statename === 'RUNNING' ? 'text-secondary-500 font-bold' : isFatal ? 'text-error-500 font-bold' : 'text-surface-900-100 font-bold'}">
                            <span class="w-2 h-2 rounded-full {process.statename === 'RUNNING' ? 'bg-secondary-500 shadow-[0_0_8px] shadow-current' : isFatal ? 'bg-error-500 shadow-[0_0_8px] shadow-current' : 'bg-surface-500'}"></span>
                            {process.statename}
                          </span>
                        </td>
                      {/if}
                      {#if columns.find(c => c.id === 'pid')?.visible}
                        <td class="px-4 py-3 font-mono text-surface-900-100 font-medium">{process.pid ?? '---'}</td>
                      {/if}
                      {#if columns.find(c => c.id === 'uptime')?.visible}
                        <td class="px-4 py-3 text-surface-900-100 font-medium">
                          {formatDistanceToNowStrict(new Date(process.start * 1000))}
                        </td>
                      {/if}
                      <td class="px-4 py-3 text-right">
                        <div class="flex items-center justify-end gap-1 opacity-70 group-hover:opacity-100 transition-opacity">
                          {#if process.statename === 'RUNNING'}
                            <button class="p-1.5 rounded-md text-surface-900-100 hover:text-secondary-500 hover:bg-surface-500/20 transition-colors" title="Restart"><RotateCcw size="16" /></button>
                            <button class="p-1.5 rounded-md text-surface-900-100 hover:text-error-500 hover:bg-surface-500/20 transition-colors" title="Stop"><Square size="16" /></button>
                          {:else}
                            <button class="p-1.5 rounded-md text-surface-900-100 hover:text-secondary-500 hover:bg-surface-500/20 transition-colors" title="Start"><Play size="16" /></button>
                          {/if}
                          <button class="p-1.5 rounded-md text-surface-900-100 hover:bg-surface-500/20 transition-colors" title="Logs"><Terminal size="16" /></button>
                          <button class="p-1.5 rounded-md text-surface-900-100 hover:bg-surface-500/20 transition-colors" title="Config"><Settings size="16" /></button>
                        </div>
                      </td>
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
</div>

<!-- Column Configuration Modal -->
{#if columnConfigOpen}
  <div class="fixed inset-0 z-[100] flex items-center justify-center bg-black/60 backdrop-blur-sm">
    <div class="w-full max-w-md overflow-hidden rounded-xl border border-surface-500/20 bg-surface-100-900 shadow-2xl">
      <div class="flex items-center justify-between border-b border-surface-500/10 px-6 py-4">
        <h3 class="text-lg font-bold tracking-tight">Configure Columns</h3>
        <button class="rounded-full p-1 hover:bg-surface-500/10" onclick={() => (columnConfigOpen = false)}>
          <X class="size-5" />
        </button>
      </div>
      <div class="space-y-4 p-6">
        <p class="text-xs font-semibold tracking-widest text-surface-500 uppercase">Visible Columns</p>
        <div class="space-y-2">
          {#each columns as col (col.id)}
            <label
              class="flex items-center justify-between rounded-lg border border-surface-500/10 bg-surface-500/5 p-3 {col.locked
                ? 'cursor-not-allowed opacity-50'
                : 'cursor-pointer hover:bg-surface-500/10'}"
            >
              <div class="flex items-center gap-3">
                <input type="checkbox" bind:checked={col.visible} disabled={col.locked} class="checkbox" />
                <span class="text-sm font-medium">{col.label}</span>
              </div>
              {#if col.locked}
                <Settings size="14" class="text-surface-500" />
              {/if}
            </label>
          {/each}
        </div>
      </div>
      <div class="flex justify-end gap-3 bg-surface-500/5 px-6 py-4">
        <button class="preset-tonal-surface-500 btn" onclick={() => (columnConfigOpen = false)}>Close</button>
      </div>
    </div>
  </div>
{/if}

<style>
  @reference '../layout.css';

  /* Local overrides for Skeleton/Tailwind 4 if needed */
  .select,
  .input {
    @apply rounded-lg px-3 py-2 transition-all focus:ring-2 focus:ring-primary-500/50 focus:outline-hidden;
  }
</style>
