<script lang="ts">

import { ListFilter, Search } from "@lucide/svelte";
import type { Host } from "$lib/api/host";
import  { PROCESS_STATES, type ProcessState } from "$lib/common";

type Prop = {
  selectedHostId: number | null;
  selectedProcessState: ProcessState | null;
  serviceRegex: string;
  hosts: Host[];
};

let {
  selectedHostId = $bindable(null),
  selectedProcessState = $bindable(null),
  serviceRegex = $bindable(""),
  hosts,
}: Prop = $props();

</script>

<div class="grid grid-cols-1 md:grid-cols-2 lg:flex lg:flex-row lg:flex-wrap gap-4 w-full">
  <!-- Host Selector -->
  <select
    bind:value={selectedHostId}
    class="select flex-1 lg:flex-none lg:w-48 lg:shrink-0 bg-surface-50-950/40 border-surface-500/30 backdrop-blur-sm"
  >
    <option value={null} class="text-surface-500/50">All Hosts</option>
    {#each hosts as host (host.id)}
      <option value={host.id}>{host.name}</option>
    {/each}
  </select>

  <!-- State Selector -->
  <select
    bind:value={selectedProcessState}
    class="select flex-1 lg:flex-none lg:w-48 lg:shrink-0 bg-surface-50-950/40 border-surface-500/30 backdrop-blur-sm"
  >
    <option value={null} class="text-surface-500/50">All States</option>
    {#each PROCESS_STATES as state (state)}
      <option value={state}>{state}</option>
    {/each}
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
  <!--  TODO -->
  <button class="btn preset-filled-surface-500/20 hover:preset-filled-surface-500/40 flex items-center justify-center gap-2 backdrop-blur-sm border border-surface-500/20 md:col-span-2 lg:col-auto lg:ml-auto lg:shrink-0">
    <ListFilter class="size-4" />
    Bulk Action
  </button>
</div>
