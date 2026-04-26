<script lang="ts">
  import {
    CircleAlert,
    ChevronDown,
    CirclePlay,
    Columns2,
    Play,
    RotateCcw,
    Server,
    Settings,
    Square,
    CircleStop,
    Terminal,
    X
  } from "@lucide/svelte";
  import { api } from "$lib";
  import type { Group } from "$lib/api/group";
  import StatCard from "./StatCard.svelte";
  import GroupSelector from "./GroupSelector.svelte";
  import type { ProcessInfo, ProcessResponse } from "$lib/api/process";
  import Filters from "./Filters.svelte";
  import type { ProcessState } from "$lib/common";
  import type { Host } from "$lib/api/host";

  // Mock data for the dashboard
  const globalStats = {
    running: 142,
    runningTrend: "+12",
    fatal: 3,
    fatalTrend: "-1",
    stopped: 18
  };

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

  $effect(() => {
    api.process.list(selectedGroupId, selectedHostId, null).then(
      (processResps: ProcessResponse[]) => {
        processInfoByHost.clear();
        for (const resp of processResps) {
          let infos = processInfoByHost.get(resp.host_id);
          if (infos === undefined) {
            infos = [];
            processInfoByHost.set(resp.host_id, infos);
          }
          infos.push(resp.process);
        }
      }
    );
  });

  $effect(() => {
    api.host.list(selectedGroupId).then(data => {
      hosts = data;
      selectedHostId = null;
      selectedProcessState = null;
    });
  });

  let collapsedHosts = $state<Record<string, boolean>>({});
  let columnConfigOpen = $state(false);

  const columns = $state([
    { id: "process", label: "Process", visible: true, locked: true },
    { id: "state", label: "State", visible: true, locked: false },
    { id: "pid", label: "PID", visible: true, locked: false },
    { id: "uptime", label: "Uptime", visible: true, locked: false },
    { id: "actions", label: "Actions", visible: true, locked: true }
  ]);

  function toggleHost(name: string) {
    collapsedHosts[name] = !collapsedHosts[name];
  }

</script>

<div class="space-y-8 pb-24">
  <!-- Global Status Dashboard (Bento style) -->
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 w-full relative z-30">
    <GroupSelector
      bind:selectedGroupId
      groups={data.groups}
    />

    <StatCard
      title="Running"
      value={globalStats.running}
      icon={CirclePlay}
      colorVariant="success"
    />

    <StatCard
      title="Fatal"
      value={globalStats.fatal}
      icon={CircleAlert}
      colorVariant="error"
    />

    <StatCard
      title="Exit / Stopped"
      value={globalStats.stopped}
      icon={CircleStop}
      colorVariant="tertiary"
    />
  </div>

  <!-- Filters & Selectors -->
  <Filters
    {hosts}
    bind:serviceRegex
    bind:selectedProcessState
    bind:selectedHostId
  />

  <!-- Host List -->
<!--  <div class="space-y-6">-->
<!--    {#each hosts as host}-->
<!--      {@const isCollapsed = !!collapsedHosts[host.name]}-->
<!--      <div class="overflow-hidden rounded-xl border border-surface-500/50 bg-surface-200-800 shadow-2xl">-->
<!--        &lt;!&ndash; Host Header (Clickable Toggle) &ndash;&gt;-->
<!--        <button -->
<!--          onclick={() => toggleHost(host.name)}-->
<!--          class="flex w-full items-center justify-between border-l-4 p-4 transition-colors hover:bg-surface-500/20 text-left {host.stats.offline ? 'border-surface-500' : 'border-secondary-500'}"-->
<!--        >-->
<!--          <div class="flex flex-wrap items-center gap-6">-->
<!--            <div class="flex items-center gap-4">-->
<!--              <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-surface-500/30 shrink-0">-->
<!--                <Server class={host.stats.offline ? 'text-surface-500' : 'text-secondary-500'} size="24" />-->
<!--              </div>-->
<!--              <div>-->
<!--                <h3 class="text-lg font-bold {host.stats.offline ? 'text-surface-500' : 'text-surface-900-100'}">{host.name}</h3>-->
<!--                <p class="font-mono text-xs text-surface-900-100 font-medium">{host.ip} • {host.region}</p>-->
<!--              </div>-->
<!--            </div>-->

<!--            <div class="flex gap-2">-->
<!--              {#if host.stats.offline}-->
<!--                <span class="flex items-center rounded-md bg-surface-500/10 px-2.5 py-1 text-xs font-bold text-surface-500 uppercase tracking-widest">-->
<!--                  Offline-->
<!--                </span>-->
<!--              {:else}-->
<!--                <span class="flex items-center rounded-md bg-secondary-500/10 px-2.5 py-1 text-xs font-bold text-secondary-500 uppercase tracking-widest">-->
<!--                  <span class="mr-1.5 h-1.5 w-1.5 animate-pulse rounded-full bg-secondary-500"></span>-->
<!--                  {host.stats.running} Running-->
<!--                </span>-->
<!--                {#if host.stats.fatal > 0}-->
<!--                  <span class="flex items-center rounded-md bg-error-500/10 px-2.5 py-1 text-xs font-bold text-error-500 uppercase tracking-widest">-->
<!--                    <span class="mr-1.5 h-1.5 w-1.5 rounded-full bg-error-500"></span>-->
<!--                    {host.stats.fatal} Fatal-->
<!--                  </span>-->
<!--                {/if}-->
<!--                {#if host.stats.stopped > 0}-->
<!--                  <span class="flex items-center rounded-md bg-surface-500/20 px-2.5 py-1 text-xs font-bold text-surface-500 uppercase tracking-widest">-->
<!--                    {host.stats.stopped} Stopped-->
<!--                  </span>-->
<!--                {/if}-->
<!--              {/if}-->
<!--            </div>-->
<!--          </div>-->

<!--          <div class="flex items-center gap-2 text-surface-500">-->
<!--            <span class="text-xs font-medium uppercase tracking-widest hidden sm:inline">-->
<!--              {isCollapsed ? 'Expand' : 'Collapse'}-->
<!--            </span>-->
<!--            <ChevronDown class="size-5 transition-transform duration-300 {isCollapsed ? '' : 'rotate-180'}" />-->
<!--          </div>-->
<!--        </button>-->

<!--        &lt;!&ndash; Processes Table (Collapsible) &ndash;&gt;-->
<!--        {#if !isCollapsed && !host.stats.offline && host.processes.length > 0}-->
<!--          <div class="bg-surface-100-900/60 p-4 border-t border-surface-500/50 overflow-x-auto">-->
<!--            <table class="table w-full text-left border-collapse">-->
<!--              <thead>-->
<!--                <tr class="bg-surface-300-700/50 text-xs text-surface-900-100 font-black uppercase tracking-widest border-b border-surface-500/50">-->
<!--                  {#if columns.find(c => c.id === 'process')?.visible}<th class="px-4 py-3 font-medium">Process</th>{/if}-->
<!--                  {#if columns.find(c => c.id === 'state')?.visible}<th class="px-4 py-3 font-medium">State</th>{/if}-->
<!--                  {#if columns.find(c => c.id === 'pid')?.visible}<th class="px-4 py-3 font-medium">PID</th>{/if}-->
<!--                  {#if columns.find(c => c.id === 'uptime')?.visible}<th class="px-4 py-3 font-medium">Uptime</th>{/if}-->
<!--                  <th class="px-4 py-3 font-medium text-right">-->
<!--                    <div class="flex items-center justify-end gap-2">-->
<!--                      Actions-->
<!--                      <button -->
<!--                        onclick={() => columnConfigOpen = true}-->
<!--                        class="p-1 hover:bg-surface-500/10 rounded transition-colors text-surface-500 hover:text-primary-500"-->
<!--                      >-->
<!--                        <Columns2 size="14" />-->
<!--                      </button>-->
<!--                    </div>-->
<!--                  </th>-->
<!--                </tr>-->
<!--              </thead>-->
<!--              <tbody>-->
<!--                {#each host.processes as process}-->
<!--                  {@const isFatal = process.state === 'FATAL'}-->
<!--                  <tr class="border-b border-surface-500/5 hover:bg-surface-500/5 transition-colors group">-->
<!--                    {#if columns.find(c => c.id === 'process')?.visible}-->
<!--                      <td class="px-4 py-3 font-medium {isFatal ? 'text-error-500' : 'text-surface-900-100'}">{process.name}</td>-->
<!--                    {/if}-->
<!--                    {#if columns.find(c => c.id === 'state')?.visible}-->
<!--                      <td class="px-4 py-3">-->
<!--                        <span class="flex items-center gap-2 {process.state === 'RUNNING' ? 'text-secondary-500 font-bold' : isFatal ? 'text-error-500 font-bold' : 'text-surface-900-100 font-bold'}">-->
<!--                          <span class="w-2 h-2 rounded-full {process.state === 'RUNNING' ? 'bg-secondary-500 shadow-[0_0_8px] shadow-current' : isFatal ? 'bg-error-500 shadow-[0_0_8px] shadow-current' : 'bg-surface-500'}"></span>-->
<!--                          {process.state}-->
<!--                        </span>-->
<!--                      </td>-->
<!--                    {/if}-->
<!--                    {#if columns.find(c => c.id === 'pid')?.visible}-->
<!--                      <td class="px-4 py-3 font-mono text-surface-900-100 font-medium">{process.pid ?? '-&#45;&#45;'}</td>-->
<!--                    {/if}-->
<!--                    {#if columns.find(c => c.id === 'uptime')?.visible}-->
<!--                      <td class="px-4 py-3 text-surface-900-100 font-medium">{process.uptime}</td>-->
<!--                    {/if}-->
<!--                    <td class="px-4 py-3 text-right">-->
<!--                      <div class="flex items-center justify-end gap-1 opacity-70 group-hover:opacity-100 transition-opacity">-->
<!--                        {#if process.state === 'RUNNING'}-->
<!--                          <button class="p-1.5 rounded-md text-surface-900-100 hover:text-secondary-500 hover:bg-surface-500/20 transition-colors" title="Restart"><RotateCcw size="16" /></button>-->
<!--                          <button class="p-1.5 rounded-md text-surface-900-100 hover:text-error-500 hover:bg-surface-500/20 transition-colors" title="Stop"><Square size="16" /></button>-->
<!--                        {:else}-->
<!--                          <button class="p-1.5 rounded-md text-surface-900-100 hover:text-secondary-500 hover:bg-surface-500/20 transition-colors" title="Start"><Play size="16" /></button>-->
<!--                        {/if}-->
<!--                        <button class="p-1.5 rounded-md text-surface-900-100 hover:bg-surface-500/20 transition-colors" title="Logs"><Terminal size="16" /></button>-->
<!--                        <button class="p-1.5 rounded-md text-surface-900-100 hover:bg-surface-500/20 transition-colors" title="Config"><Settings size="16" /></button>-->
<!--                      </div>-->
<!--                    </td>-->
<!--                  </tr>-->
<!--                {/each}-->
<!--              </tbody>-->
<!--            </table>-->
<!--          </div>-->
<!--        {/if}-->
<!--      </div>-->
<!--    {/each}-->
<!--  </div>-->
</div>

<!-- Column Configuration Modal -->
{#if columnConfigOpen}
  <div class="fixed inset-0 z-[100] flex items-center justify-center bg-black/60 backdrop-blur-sm">
    <div class="bg-surface-100-900 w-full max-w-md rounded-xl shadow-2xl border border-surface-500/20 overflow-hidden">
      <div class="px-6 py-4 border-b border-surface-500/10 flex items-center justify-between">
        <h3 class="font-bold text-lg tracking-tight">Configure Columns</h3>
        <button class="p-1 hover:bg-surface-500/10 rounded-full" onclick={() => columnConfigOpen = false}>
          <X class="size-5" />
        </button>
      </div>
      <div class="p-6 space-y-4">
        <p class="text-xs text-surface-500 uppercase tracking-widest font-semibold">Visible Columns</p>
        <div class="space-y-2">
          {#each columns as col}
            <label class="flex items-center justify-between p-3 bg-surface-500/5 rounded-lg border border-surface-500/10 {col.locked ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer hover:bg-surface-500/10'}">
              <div class="flex items-center gap-3">
                <input 
                  type="checkbox" 
                  bind:checked={col.visible} 
                  disabled={col.locked}
                  class="checkbox"
                />
                <span class="text-sm font-medium">{col.label}</span>
              </div>
              {#if col.locked}
                <Settings size="14" class="text-surface-500" />
              {/if}
            </label>
          {/each}
        </div>
      </div>
      <div class="px-6 py-4 bg-surface-500/5 flex justify-end gap-3">
        <button class="btn preset-tonal-surface-500" onclick={() => columnConfigOpen = false}>Close</button>
      </div>
    </div>
  </div>
{/if}

<style>
  @reference '../layout.css';

  /* Local overrides for Skeleton/Tailwind 4 if needed */
  .select, .input {
    @apply rounded-lg py-2 px-3 focus:ring-2 focus:ring-primary-500/50 focus:outline-hidden transition-all;
  }
</style>
