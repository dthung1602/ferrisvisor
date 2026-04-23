<script lang="ts">
  import {
    AlertCircle,
    ChevronDown,
    CirclePlay,
    Columns2,
    Globe,
    ListFilter,
    Play,
    PlusCircle,
    RotateCcw,
    Search,
    Server,
    Settings,
    Square,
    StopCircle,
    Terminal,
    X
  } from "@lucide/svelte";

  // Mock data for the dashboard
  const globalStats = {
    running: 142,
    runningTrend: "+12",
    fatal: 3,
    fatalTrend: "-1",
    stopped: 18
  };

  const hosts = [
    {
      name: "node-alpha-01",
      ip: "192.168.1.104",
      region: "us-east-1a",
      stats: { running: 8, fatal: 1, stopped: 0 },
      processes: [
        { name: "api-gateway-service", state: "RUNNING", pid: 4892, uptime: "14d 02:44:12" },
        { name: "payment-processor-worker", state: "FATAL", pid: null, uptime: "---" },
        { name: "redis-cache", state: "RUNNING", pid: 892, uptime: "45d 12h 10m" }
      ]
    },
    {
      name: "node-beta-04",
      ip: "10.1.0.82",
      region: "us-east-1b",
      stats: { running: 4, fatal: 0, stopped: 2 },
      processes: [
        { name: "auth-service-v2", state: "STOPPED", pid: null, uptime: "0s" },
        { name: "inventory-manager", state: "RUNNING", pid: 1056, uptime: "2d 04:12:33" }
      ]
    },
    {
      name: "node-gamma-09",
      ip: "192.168.1.106",
      region: "us-east-1c",
      stats: { offline: true },
      processes: []
    }
  ];

  let selectedEnv = $state("production");
  let selectedHost = $state("");
  let serviceRegex = $state("");
  let selectedState = $state("All States");

  const environments = [
    { id: "production", label: "Production", color: "text-secondary-500", dot: "bg-secondary-500" },
    { id: "staging", label: "Staging", color: "text-tertiary-500", dot: "bg-tertiary-500" },
    { id: "development", label: "Development", color: "text-primary-500", dot: "bg-primary-500" }
  ];

  let envDropdownOpen = $state(false);
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

  function selectEnv(id: string) {
    selectedEnv = id;
    envDropdownOpen = false;
  }
</script>

<div class="space-y-8 pb-24">
  <!-- Global Status Dashboard (Bento style) -->
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 w-full relative z-30">
    <!-- Environment Selector Card (First Cell) -->
    <div class="group relative overflow-visible rounded-xl p-5 border transition-all z-50 flex flex-col justify-between min-h-[140px]
      {selectedEnv === 'production' ? 'bg-secondary-500/15 border-secondary-500/30' : 
       selectedEnv === 'staging' ? 'bg-tertiary-500/15 border-tertiary-500/30' : 
       'bg-primary-500/15 border-primary-500/30'}">
      <div class="mb-2 flex justify-between items-start">
        <span class="text-xs font-bold tracking-widest uppercase
          {selectedEnv === 'production' ? 'text-secondary-500' : 
           selectedEnv === 'staging' ? 'text-tertiary-500' : 
           'text-primary-500'}">Environment</span>
        <Globe class="size-5 
          {selectedEnv === 'production' ? 'text-secondary-500' : 
           selectedEnv === 'staging' ? 'text-tertiary-500' : 
           'text-primary-500'}" />
      </div>
      
      <div class="relative">
        <button 
          onclick={() => envDropdownOpen = !envDropdownOpen}
          class="flex w-full items-center justify-between rounded-lg bg-black/20 px-4 py-3 transition-all hover:bg-black/30 active:scale-[0.98]"
        >
          <div class="flex items-center gap-3">
            <span class="h-2 w-2 rounded-full {environments.find(e => e.id === selectedEnv)?.dot} shadow-[0_0_8px] shadow-current"></span>
            <span class="text-lg font-black tracking-tight text-surface-900-100">
              {environments.find(e => e.id === selectedEnv)?.label}
            </span>
          </div>
          <ChevronDown class="size-4 text-surface-900-100 transition-transform {envDropdownOpen ? 'rotate-180' : ''}" />
        </button>

        {#if envDropdownOpen}
          <div class="absolute left-0 top-full z-[100] mt-2 w-full overflow-hidden rounded-xl border border-surface-500/20 bg-surface-100-900 shadow-2xl backdrop-blur-xl">
            {#each environments as env}
              <button 
                onclick={() => selectEnv(env.id)}
                class="flex w-full items-center gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-500/10 {selectedEnv === env.id ? 'bg-primary-500/10 font-bold' : ''}"
              >
                <span class="h-2 w-2 rounded-full {env.dot}"></span>
                <span class="text-sm {selectedEnv === env.id ? 'text-primary-500' : 'text-surface-600-400'}">{env.label}</span>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    </div>

    <!-- Running Card -->
    <div class="group relative overflow-hidden rounded-xl bg-surface-50-950/40 p-5 backdrop-blur-sm border border-surface-500/10 transition-all hover:bg-surface-500/10 flex flex-col justify-between min-h-[140px]">
      <div class="absolute inset-0 bg-linear-to-br from-secondary-500/10 to-transparent opacity-0 transition-opacity group-hover:opacity-100"></div>
      <div class="mb-2 flex justify-between items-start">
        <span class="text-xs font-bold tracking-widest text-surface-500 uppercase">Running</span>
        <CirclePlay class="text-secondary-500 size-6" />
      </div>
      <div class="flex items-baseline gap-2">
        <span class="text-4xl font-bold text-secondary-500">{globalStats.running}</span>
        <span class="rounded-full bg-secondary-500/10 px-2 py-0.5 text-xs font-medium text-secondary-500">
          {globalStats.runningTrend}
        </span>
      </div>
    </div>

    <!-- Fatal Card -->
    <div class="group relative overflow-hidden rounded-xl bg-surface-50-950/40 p-5 backdrop-blur-sm border border-surface-500/10 transition-all hover:bg-surface-500/10 flex flex-col justify-between min-h-[140px]">
      <div class="absolute inset-0 bg-linear-to-br from-error-500/10 to-transparent opacity-0 transition-opacity group-hover:opacity-100"></div>
      <div class="mb-2 flex justify-between items-start">
        <span class="text-xs font-bold tracking-widest text-surface-500 uppercase">Fatal</span>
        <AlertCircle class="text-error-500 size-6" />
      </div>
      <div class="flex items-baseline gap-2">
        <span class="text-4xl font-bold text-error-500">{globalStats.fatal}</span>
        <span class="rounded-full bg-error-500/10 px-2 py-0.5 text-xs font-medium text-error-500">
          {globalStats.fatalTrend}
        </span>
      </div>
    </div>

    <!-- Stopped Card -->
    <div class="group relative overflow-hidden rounded-xl bg-surface-50-950/40 p-5 backdrop-blur-sm border border-surface-500/10 transition-all hover:bg-surface-500/10 flex flex-col justify-between min-h-[140px]">
      <div class="mb-2 flex justify-between items-start">
        <span class="text-xs font-bold tracking-widest text-surface-500 uppercase">Stopped</span>
        <StopCircle class="text-surface-500 size-6" />
      </div>
      <div class="flex items-baseline gap-2">
        <span class="text-4xl font-bold text-surface-900-100">{globalStats.stopped}</span>
      </div>
    </div>
  </div>

  <!-- Filters & Selectors -->
  <div class="grid grid-cols-1 md:grid-cols-2 lg:flex lg:flex-row lg:flex-wrap gap-4 w-full">
    <!-- Host Selector -->
    <select bind:value={selectedHost} class="select flex-1 lg:flex-none lg:w-48 lg:shrink-0 bg-surface-50-950/40 border-surface-500/30 backdrop-blur-sm">
      <option value="">Select Host</option>
      {#each hosts as host}
        <option value={host.name}>{host.name}</option>
      {/each}
    </select>

    <!-- State Selector -->
    <select bind:value={selectedState} class="select flex-1 lg:flex-none lg:w-48 lg:shrink-0 bg-surface-50-950/40 border-surface-500/30 backdrop-blur-sm">
      <option>All States</option>
      <option>Running</option>
      <option>Stopped</option>
      <option>Fatal</option>
    </select>

    <!-- Search Input -->
    <div class="relative flex-1 md:col-span-2 lg:flex-1 lg:min-w-64">
      <Search class="absolute left-3 top-1/2 -translate-y-1/2 text-surface-500 size-4" />
      <input
        type="text"
        bind:value={serviceRegex}
        placeholder="Service Regex..."
        class="input w-full pl-10 bg-surface-50-950/40 border-surface-500/30 backdrop-blur-sm"
      />
    </div>

    <!-- Bulk Action -->
    <button class="btn preset-filled-surface-500/20 hover:preset-filled-surface-500/40 flex items-center justify-center gap-2 backdrop-blur-sm border border-surface-500/20 md:col-span-2 lg:col-auto lg:ml-auto lg:shrink-0">
      <ListFilter class="size-4" />
      Bulk Action
    </button>
  </div>

  <!-- Host List -->
  <div class="space-y-6">
    {#each hosts as host}
      {@const isCollapsed = !!collapsedHosts[host.name]}
      <div class="overflow-hidden rounded-xl border border-surface-500/50 bg-surface-200-800 shadow-2xl">
        <!-- Host Header (Clickable Toggle) -->
        <button 
          onclick={() => toggleHost(host.name)}
          class="flex w-full items-center justify-between border-l-4 p-4 transition-colors hover:bg-surface-500/20 text-left {host.stats.offline ? 'border-surface-500' : 'border-secondary-500'}"
        >
          <div class="flex flex-wrap items-center gap-6">
            <div class="flex items-center gap-4">
              <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-surface-500/30 shrink-0">
                <Server class={host.stats.offline ? 'text-surface-500' : 'text-secondary-500'} size="24" />
              </div>
              <div>
                <h3 class="text-lg font-bold {host.stats.offline ? 'text-surface-500' : 'text-surface-900-100'}">{host.name}</h3>
                <p class="font-mono text-xs text-surface-900-100 font-medium">{host.ip} • {host.region}</p>
              </div>
            </div>

            <div class="flex gap-2">
              {#if host.stats.offline}
                <span class="flex items-center rounded-md bg-surface-500/10 px-2.5 py-1 text-xs font-bold text-surface-500 uppercase tracking-widest">
                  Offline
                </span>
              {:else}
                <span class="flex items-center rounded-md bg-secondary-500/10 px-2.5 py-1 text-xs font-bold text-secondary-500 uppercase tracking-widest">
                  <span class="mr-1.5 h-1.5 w-1.5 animate-pulse rounded-full bg-secondary-500"></span>
                  {host.stats.running} Running
                </span>
                {#if host.stats.fatal > 0}
                  <span class="flex items-center rounded-md bg-error-500/10 px-2.5 py-1 text-xs font-bold text-error-500 uppercase tracking-widest">
                    <span class="mr-1.5 h-1.5 w-1.5 rounded-full bg-error-500"></span>
                    {host.stats.fatal} Fatal
                  </span>
                {/if}
                {#if host.stats.stopped > 0}
                  <span class="flex items-center rounded-md bg-surface-500/20 px-2.5 py-1 text-xs font-bold text-surface-500 uppercase tracking-widest">
                    {host.stats.stopped} Stopped
                  </span>
                {/if}
              {/if}
            </div>
          </div>

          <div class="flex items-center gap-2 text-surface-500">
            <span class="text-xs font-medium uppercase tracking-widest hidden sm:inline">
              {isCollapsed ? 'Expand' : 'Collapse'}
            </span>
            <ChevronDown class="size-5 transition-transform duration-300 {isCollapsed ? '' : 'rotate-180'}" />
          </div>
        </button>

        <!-- Processes Table (Collapsible) -->
        {#if !isCollapsed && !host.stats.offline && host.processes.length > 0}
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
                {#each host.processes as process}
                  {@const isFatal = process.state === 'FATAL'}
                  <tr class="border-b border-surface-500/5 hover:bg-surface-500/5 transition-colors group">
                    {#if columns.find(c => c.id === 'process')?.visible}
                      <td class="px-4 py-3 font-medium {isFatal ? 'text-error-500' : 'text-surface-900-100'}">{process.name}</td>
                    {/if}
                    {#if columns.find(c => c.id === 'state')?.visible}
                      <td class="px-4 py-3">
                        <span class="flex items-center gap-2 {process.state === 'RUNNING' ? 'text-secondary-500 font-bold' : isFatal ? 'text-error-500 font-bold' : 'text-surface-900-100 font-bold'}">
                          <span class="w-2 h-2 rounded-full {process.state === 'RUNNING' ? 'bg-secondary-500 shadow-[0_0_8px] shadow-current' : isFatal ? 'bg-error-500 shadow-[0_0_8px] shadow-current' : 'bg-surface-500'}"></span>
                          {process.state}
                        </span>
                      </td>
                    {/if}
                    {#if columns.find(c => c.id === 'pid')?.visible}
                      <td class="px-4 py-3 font-mono text-surface-900-100 font-medium">{process.pid ?? '---'}</td>
                    {/if}
                    {#if columns.find(c => c.id === 'uptime')?.visible}
                      <td class="px-4 py-3 text-surface-900-100 font-medium">{process.uptime}</td>
                    {/if}
                    <td class="px-4 py-3 text-right">
                      <div class="flex items-center justify-end gap-1 opacity-70 group-hover:opacity-100 transition-opacity">
                        {#if process.state === 'RUNNING'}
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
    {/each}
  </div>
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
  /* Local overrides for Skeleton/Tailwind 4 if needed */
  .select, .input {
    @apply rounded-lg py-2 px-3 focus:ring-2 focus:ring-primary-500/50 focus:outline-hidden transition-all;
  }
</style>
