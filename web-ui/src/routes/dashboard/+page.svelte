<script lang="ts">
  import { untrack } from "svelte";

  import { CircleAlert, CirclePlay, CircleStop } from "@lucide/svelte";
  import { browser } from "$app/environment";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { api, localstorage } from "$lib";
  import { debounce } from "lodash";
  import { SvelteMap } from "svelte/reactivity";

  import type { Group } from "$lib/api/group";
  import type { Host } from "$lib/api/host";
  import type { ProcessActionRequest, ProcessInfo, ProcessResponse } from "$lib/api/process";
  import { PROCESS_STATES, type ProcessState } from "$lib/constants";

  import ColumnConfigModal from "./ColumnConfigModal.svelte";
  import type { ProcessColumn } from "./common.ts";
  import Filters from "./Filters.svelte";
  import GroupCard from "./GroupCard.svelte";
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

  const getInitialGroupId = () => {
    if (!browser) return data.groups[0]?.id ?? null;
    const val = page.url.searchParams.get("group");
    if (val !== null) {
      const groupObj = data.groups.find((g) => g.name === val);
      if (groupObj) {
        return groupObj.id;
      }
    }
    return data.groups[0]?.id ?? null;
  };

  const getInitialHostId = () => {
    if (!browser) return null;
    const val = page.url.searchParams.get("host");
    if (val !== null) {
      const parsed = Number(val);
      if (!isNaN(parsed)) {
        return parsed;
      }
    }
    return null;
  };

  const getInitialProcessState = () => {
    if (!browser) return null;
    const val = page.url.searchParams.get("state");
    if (val !== null && (PROCESS_STATES as readonly string[]).includes(val)) {
      return val as ProcessState;
    }
    return null;
  };

  const getInitialRegex = () => {
    if (!browser) return "";
    return page.url.searchParams.get("regex") ?? "";
  };

  let selectedGroupId: number | null = $state(getInitialGroupId());
  let selectedHostId: number | null = $state(getInitialHostId());
  let serviceRegex = $state(getInitialRegex());
  let selectedProcessState: ProcessState | null = $state(getInitialProcessState());

  // Debounced regex for URL synchronization to prevent performance lag while typing
  let debouncedServiceRegex = $state(getInitialRegex());

  const updateDebouncedRegex = debounce((val: string) => {
    debouncedServiceRegex = val;
  }, 200);

  $effect(() => {
    updateDebouncedRegex(serviceRegex);
    return () => {
      updateDebouncedRegex.cancel();
    };
  });

  // eslint-disable-next-line svelte/prefer-writable-derived
  let isInitialized = $state(false);

  $effect(() => {
    isInitialized = true;
  });

  let hosts: Host[] = $state([]);
  let processInfoByHost = $state(new Map<number, ProcessInfo[]>());
  let selectedProcesses = $state<ProcessActionRequest[]>([]);

  let columnConfigOpen = $state(false);

  const DEFAULT_COLUMNS: ProcessColumn[] = [
    { id: "process", label: "Process", visible: true, locked: true },
    { id: "state", label: "State", visible: true, locked: false },
    { id: "pid", label: "PID", visible: true, locked: false },
    { id: "last_changed", label: "Last changed", visible: true, locked: false },
    { id: "actions", label: "Actions", visible: true, locked: true }
  ];

  function loadColumns(): ProcessColumn[] {
    if (!browser) return [...DEFAULT_COLUMNS];

    try {
      const saved = localstorage.get(localstorage.DASHBOARD_COLUMNS) as { id: string; visible: boolean }[];
      if (!saved) return [...DEFAULT_COLUMNS];

      // Reconstruct columns while maintaining order from saved data
      const newColumns: ProcessColumn[] = [];

      // Add saved columns in their saved order
      for (const savedCol of saved) {
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
    localstorage.set(localstorage.DASHBOARD_COLUMNS, toSave);
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

  let openHostIds = $state<string[]>([]);

  let filteredHostIds = $derived.by(() => {
    return hosts
      .filter((host) => {
        const hostProcesses = processInfoByHost.get(host.id) ?? [];

        // Match process filters
        const filteredProcesses = hostProcesses.filter((p) => {
          if (serviceRegex && !p.name.match(serviceRegex)) {
            return false;
          }
          if (selectedProcessState && p.statename !== selectedProcessState) {
            return false;
          }
          return true;
        });

        // Compute host stats to see if any processes match the state filter
        const stats = Object.fromEntries(PROCESS_STATES.map((state) => [state, 0])) as Record<ProcessState, number>;
        for (let process of hostProcesses) {
          stats[process.statename]++;
        }

        return (
          (!selectedHostId || selectedHostId === host.id) &&
          (!selectedProcessState || stats[selectedProcessState] > 0) &&
          filteredProcesses.length > 0
        );
      })
      .map((h) => h.id.toString());
  });

  let lastFilteredIds: string[] = [];
  $effect(() => {
    if (
      filteredHostIds.length !== lastFilteredIds.length ||
      !filteredHostIds.every((id, idx) => id === lastFilteredIds[idx])
    ) {
      openHostIds = [...filteredHostIds];
      lastFilteredIds = [...filteredHostIds];
    }
  });

  function setAllHostPanelCollapseState(collapsed: boolean) {
    if (collapsed) {
      openHostIds = [];
    } else {
      openHostIds = [...filteredHostIds];
    }
  }

  // fetch data when filter changes
  $effect(() => {
    if (!isInitialized) return;
    refreshAllProcessInfo();
  });

  let isFirstHostLoad = true;

  // fetch hosts when group changes
  $effect(() => {
    if (!isInitialized) return;

    const gId = selectedGroupId;

    untrack(() => {
      selectedProcesses = [];
      api.host.list(gId).then((data) => {
        hosts = data;
        if (isFirstHostLoad) {
          isFirstHostLoad = false;
          // Verify if the initial host ID from the URL is actually in the fetched hosts
          if (selectedHostId !== null && !hosts.some((h) => h.id === selectedHostId)) {
            selectedHostId = null;
            selectedProcessState = null;
          }
        } else {
          selectedHostId = null;
          selectedProcessState = null;
        }
      });
    });
  });

  // State -> URL synchronization
  $effect(() => {
    if (!isInitialized || !browser) return;

    const gId = selectedGroupId;
    const hId = selectedHostId;
    const pState = selectedProcessState;
    const regex = debouncedServiceRegex;

    const url = new URL(page.url);

    const groupObj = data.groups.find((g) => g.id === gId);
    const gName = groupObj?.name ?? null;

    if (gName !== null) {
      url.searchParams.set("group", gName);
    } else {
      url.searchParams.delete("group");
    }

    if (hId !== null) {
      url.searchParams.set("host", String(hId));
    } else {
      url.searchParams.delete("host");
    }

    if (pState !== null) {
      url.searchParams.set("state", pState);
    } else {
      url.searchParams.delete("state");
    }

    if (regex) {
      url.searchParams.set("regex", regex);
    } else {
      url.searchParams.delete("regex");
    }

    if (url.search !== page.url.search) {
      // eslint-disable-next-line svelte/no-navigation-without-resolve
      goto(url.pathname + url.search, {
        keepFocus: true,
        replaceState: true,
        noScroll: true
      });
    }
  });

  // URL -> State synchronization (handles back/forward navigation)
  $effect(() => {
    if (!isInitialized || !browser) return;

    const currentUrl = page.url;

    untrack(() => {
      // 1. Sync Group Name to Group ID
      const urlGroupName = currentUrl.searchParams.get("group");
      let targetGroupId = data.groups[0]?.id ?? null;
      if (urlGroupName !== null) {
        const groupObj = data.groups.find((g) => g.name === urlGroupName);
        if (groupObj) {
          targetGroupId = groupObj.id;
        }
      }
      if (selectedGroupId !== targetGroupId) {
        selectedGroupId = targetGroupId;
      }

      // 2. Sync Host ID
      const urlHost = currentUrl.searchParams.get("host");
      let targetHostId: number | null = null;
      if (urlHost !== null) {
        const parsed = Number(urlHost);
        if (!isNaN(parsed)) {
          targetHostId = parsed;
        }
      }
      if (selectedHostId !== targetHostId) {
        selectedHostId = targetHostId;
      }

      // 3. Sync Process State
      const urlState = currentUrl.searchParams.get("state");
      let targetState: ProcessState | null = null;
      if (urlState !== null && (PROCESS_STATES as readonly string[]).includes(urlState)) {
        targetState = urlState as ProcessState;
      }
      if (selectedProcessState !== targetState) {
        selectedProcessState = targetState;
      }

      // 4. Sync Regex
      const urlRegex = currentUrl.searchParams.get("regex") ?? "";
      if (serviceRegex !== urlRegex) {
        serviceRegex = urlRegex;
      }
    });
  });
</script>

<div class="space-y-8 pb-24">
  <!-- Global Status Dashboard (Bento style) -->
  <div class="relative z-30 grid w-full grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
    <GroupCard bind:selectedGroupId groups={data.groups} />

    <StatCard title="Running" value={globalStats.running} icon={CirclePlay} colorVariant="success" />

    <StatCard title="Fatal" value={globalStats.fatal} icon={CircleAlert} colorVariant="error" />

    <StatCard title="Exit / Stopped" value={globalStats.stopped} icon={CircleStop} colorVariant="tertiary" />
  </div>

  <!-- Filters & Selectors -->
  <Filters
    {hosts}
    bind:serviceRegex
    bind:selectedProcessState
    bind:selectedHostId
    bind:selectedProcesses
    {processInfoByHost}
    {refreshAllProcessInfo}
    {setAllHostPanelCollapseState}
  />

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
    bind:selectedProcesses
    bind:openHostIds
  />
</div>

<!-- Column Configuration Modal -->
<ColumnConfigModal bind:open={columnConfigOpen} bind:columns />

<style>
  @reference '../layout.css';
</style>
