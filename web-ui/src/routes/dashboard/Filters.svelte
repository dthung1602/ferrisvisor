<script lang="ts">
  import { ListFilter, Search } from "@lucide/svelte";

  import type { Host } from "$lib/api/host";
  import HostSelector from "$lib/components/HostSelector.svelte";
  import StateSelector from "$lib/components/StateSelector.svelte";
  import type { ProcessState } from "$lib/constants";

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
  <div class="relative flex-1 lg:w-48 lg:flex-none lg:shrink-0">
    <HostSelector {hosts} {selectedHostId} setSelectedHostId={(id) => (selectedHostId = id)} />
  </div>

  <!-- State Selector -->
  <div class="relative flex-1 lg:w-48 lg:flex-none lg:shrink-0">
    <StateSelector {selectedProcessState} setSelectedProcessState={(state) => (selectedProcessState = state)} />
  </div>

  <!-- Search Input -->
  <div class="relative flex-1 md:col-span-2 lg:min-w-64 lg:flex-1">
    <Search class="pointer-events-none absolute top-1/2 left-4 size-4 -translate-y-1/2 text-surface-800-200" />
    <input
      type="text"
      bind:value={serviceRegex}
      placeholder="Process Regex ..."
      class="w-full rounded-xl border-2 border-surface-200 bg-white py-3 pr-4 pl-10 text-sm transition-all placeholder:text-surface-800-200 hover:bg-surface-100/20 focus:ring-2 focus:ring-primary-500/50 focus:outline-none dark:border-surface-700 dark:bg-surface-900 dark:hover:bg-surface-500/40"
    />
  </div>

  <!-- Bulk Action -->
  <button
    class="flex items-center justify-center gap-2 rounded-xl border-2 border-surface-200 bg-white px-4 py-3 text-sm font-medium transition-all hover:bg-surface-100/20 active:scale-[0.99] md:col-span-2 lg:col-auto lg:ml-auto lg:shrink-0 dark:border-surface-700 dark:bg-surface-900 dark:hover:bg-surface-500/40"
  >
    <ListFilter class="size-4" />
    Bulk Action
  </button>
</div>
