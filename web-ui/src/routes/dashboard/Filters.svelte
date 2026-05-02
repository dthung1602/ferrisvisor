<script lang="ts">
  import { ListFilter, Search } from "@lucide/svelte";

  import type { Host } from "$lib/api/host";
  import { PROCESS_STATES, type ProcessState } from "$lib/constants";

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
    hosts
  }: Prop = $props();
</script>

<div class="grid w-full grid-cols-1 gap-4 md:grid-cols-2 lg:flex lg:flex-row lg:flex-wrap">
  <!-- Host Selector -->
  <select
    bind:value={selectedHostId}
    class="select flex-1 border-surface-500/30 bg-surface-50-950/40 backdrop-blur-sm lg:w-48 lg:flex-none lg:shrink-0"
  >
    <option value={null} class="text-surface-500/50">All Hosts</option>
    {#each hosts as host (host.id)}
      <option value={host.id}>{host.name}</option>
    {/each}
  </select>

  <!-- State Selector -->
  <select
    bind:value={selectedProcessState}
    class="select flex-1 border-surface-500/30 bg-surface-50-950/40 backdrop-blur-sm lg:w-48 lg:flex-none lg:shrink-0"
  >
    <option value={null} class="text-surface-500/50">All States</option>
    {#each PROCESS_STATES as state (state)}
      <option value={state}>{state}</option>
    {/each}
  </select>

  <!-- Search Input -->
  <div class="relative flex-1 md:col-span-2 lg:min-w-64 lg:flex-1">
    <Search class="pointer-events-none absolute top-1/2 left-3 z-10 size-4 -translate-y-1/2 text-surface-300" />
    <input
      type="text"
      bind:value={serviceRegex}
      placeholder="Process Regex ..."
      class="input w-full border-surface-500/30 bg-surface-50-950/40 backdrop-blur-sm"
      style="padding-left: 2.25rem;"
    />
  </div>

  <!-- Bulk Action -->
  <!--  TODO -->
  <button
    class="preset-filled-surface-500/20 hover:preset-filled-surface-500/40 btn flex items-center justify-center gap-2 border border-surface-500/20 backdrop-blur-sm md:col-span-2 lg:col-auto lg:ml-auto lg:shrink-0"
  >
    <ListFilter class="size-4" />
    Bulk Action
  </button>
</div>

<style>
  @reference '../layout.css';

  /* Local overrides for Skeleton/Tailwind 4 if needed */
  .select,
  .input {
    @apply rounded-lg px-3 py-2 transition-all focus:ring-2 focus:ring-primary-500/50 focus:outline-hidden;
  }
</style>
