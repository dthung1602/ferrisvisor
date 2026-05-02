<script lang="ts">
  import { browser } from "$app/environment";
  import { CircleAlert, CirclePlay, CircleStop } from "@lucide/svelte";
  import { api, cookies } from "$lib";
  import { SvelteMap } from "svelte/reactivity";

  import type { Group } from "$lib/api/group";
  import type { Host } from "$lib/api/host";
  import type { ProcessInfo, ProcessResponse } from "$lib/api/process";
  import { PROCESS_STATES, type ProcessState } from "$lib/constants";

  import type { ProcessColumn } from "./common.ts";
  import ColumnConfigModal from "./ColumnConfigModal.svelte";
  import Filters from "./Filters.svelte";
  import GroupSelector from "./GroupSelector.svelte";
  import HostList from "./HostList.svelte";
  import StatCard from "./StatCard.svelte";

  type HostStats = {
    running: number;
    stopped: number;
    fatal: number;
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

  let columnConfigOpen = $state(false);

  const DEFAULT_COLUMNS: ProcessColumn[] = [
    { id: "process", label: "Process", visible: true, locked: true },
    { id: "state", label: "State", visible: true, locked: false },
    { id: "pid", label: "PID", visible: true, locked: false },
    { id: "last_changed", label: "Last changed", visible: true, locked: false },
    { id: "actions", label: "Actions", visible: true, locked: true }
  ];

  const COLUMN_COOKIE_NAME = "dashboard_columns";

  function loadColumns(): ProcessColumn[] {
    if (!browser) return [...DEFAULT_COLUMNS];

    const saved = cookies.getCookie(COLUMN_COOKIE_NAME);
    if (!saved) return [...DEFAULT_COLUMNS];

    try {
      const parsed = JSON.parse(saved) as { id: string; visible: boolean }[];
      // Reconstruct columns while maintaining order from saved data
      const newColumns: ProcessColumn[] = [];

      // Add saved columns in their saved order
      for (const savedCol of parsed) {
        const template = DEFAULT_COLUMNS.find((c) => c.id === savedCol.id);
        if (template) {
          newColumns.push({ ...template, visible: savedCol.visible });
        }
      }

      // Add any columns from DEFAULT_COLUMNS that weren't in saved data (e.g. newly added columns)
      for (const template of DEFAULT_COLUMNS) {
        if (!newColumns.find((c) => c.id === template.id)) {
          newColumns.push({ ...template });
        }
      }

      return newColumns;
    } catch (e) {
      console.error("Failed to parse saved columns", e);
      return [...DEFAULT_COLUMNS];
    }
  }

  let columns: ProcessColumn[] = $state(loadColumns());

  $effect(() => {
    const toSave = columns.map((c) => ({ id: c.id, visible: c.visible }));
    cookies.setPersistentCookie(COLUMN_COOKIE_NAME, JSON.stringify(toSave));
  });

  function summarizeHostStats(processInfos: ProcessInfo[]): HostStats {
    const stats = Object.fromEntries(PROCESS_STATES.map((state) => [state, 0])) as Record<ProcessState, number>;
    for (let process of processInfos) {
      stats[process.statename]++;
    }

    return {
      running: stats.STARTING + stats.RUNNING + stats.STOPPING,
      fatal: stats.BACKOFF + stats.FATAL + stats.UNKNOWN,
      stopped: stats.STOPPED + stats.EXITED
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
  });

  function refreshAllProcessInfo() {
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
      processInfoByHost = newProcessInfoByHost;
    });
  }

  // fetch data when filter changes
  $effect(refreshAllProcessInfo);

  // fetch hosts when group changes
  $effect(() => {
    api.host.list(selectedGroupId).then((data) => {
      hosts = data;
      selectedHostId = null;
      selectedProcessState = null;
    });
  });
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
  <HostList
    bind:columnConfigOpen
    {columns}
    {hosts}
    {processInfoByHost}
    {selectedHostId}
    {serviceRegex}
    {selectedProcessState}
    {refreshAllProcessInfo}
  />
</div>

<!-- Column Configuration Modal -->
<ColumnConfigModal bind:open={columnConfigOpen} bind:columns />

<style>
  @reference '../layout.css';
</style>
