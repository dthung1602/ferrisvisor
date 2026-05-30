<script lang="ts">
  import { ChevronDown, ListFilter, Search } from "@lucide/svelte";
  import { Menu, Portal } from "@skeletonlabs/skeleton-svelte";

  import type { Host } from "$lib/api/host";
  import HostSelector from "$lib/components/HostSelector.svelte";
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

  let stateMenuOpen = $state(false);

  let stateSearch = $state("");

  let filteredStates = $derived(PROCESS_STATES.filter((s) => s.toLowerCase().includes(stateSearch.toLowerCase())));
</script>

<div class="grid w-full grid-cols-1 gap-4 md:grid-cols-2 lg:flex lg:flex-row lg:flex-wrap">
  <!-- Host Selector -->
  <div class="relative flex-1 lg:w-48 lg:flex-none lg:shrink-0">
    <HostSelector {hosts} {selectedHostId} setSelectedHostId={(id) => (selectedHostId = id)} />
  </div>

  <!-- State Selector -->
  <div class="relative flex-1 lg:w-48 lg:flex-none lg:shrink-0">
    <Menu
      open={stateMenuOpen}
      onOpenChange={(e) => {
        stateMenuOpen = e.open;
        if (!e.open) stateSearch = "";
      }}
      positioning={{ placement: "bottom-start", gutter: 8, sameWidth: true }}
    >
      <Menu.Trigger
        class="flex w-full items-center justify-between rounded-lg border border-surface-500/30 bg-surface-50 dark:bg-surface-950/40 px-3 py-2 backdrop-blur-sm transition-all hover:bg-surface-500/10 active:scale-[0.98]"
      >
        <span class="truncate text-sm font-medium {selectedProcessState === null ? 'text-surface-300' : ''}">
          {selectedProcessState || "All States"}
        </span>
        <ChevronDown class="size-4 shrink-0 transition-transform {stateMenuOpen ? 'rotate-180' : ''}" />
      </Menu.Trigger>
      <Portal>
        <Menu.Positioner>
          <Menu.Content
            class="z-100 overflow-hidden rounded-xl border border-surface-500 bg-surface-100-900 shadow-2xl backdrop-blur-xl"
          >
            <div class="border-b border-surface-500/10 p-2">
              <div class="relative">
                <Search class="pointer-events-none absolute top-1/2 left-3 size-3 -translate-y-1/2 opacity-40" />
                <input
                  type="text"
                  bind:value={stateSearch}
                  placeholder="Filter states..."
                  class="w-full rounded-lg border border-transparent bg-surface-500/10 py-1.5 pr-3 pl-8 text-xs ring-0 outline-none focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
                  onclick={(e) => e.stopPropagation()}
                />
              </div>
            </div>

            <div class="max-h-128 overflow-y-auto">
              {#if "all states".includes(stateSearch.toLowerCase())}
                <Menu.OptionItem
                  type="radio"
                  value="null"
                  checked={selectedProcessState === null}
                  onCheckedChange={() => (selectedProcessState = null)}
                  class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-500/20 data-[state=checked]:bg-surface-500/10"
                >
                  <Menu.ItemText class="text-sm font-medium">All States</Menu.ItemText>
                  <Menu.ItemIndicator>
                    <div class="h-1.5 w-1.5 rounded-full bg-current"></div>
                  </Menu.ItemIndicator>
                </Menu.OptionItem>
              {/if}
              {#each filteredStates as state (state)}
                <Menu.OptionItem
                  type="radio"
                  value={state}
                  checked={selectedProcessState === state}
                  onCheckedChange={(checked) => {
                    if (checked) selectedProcessState = state;
                  }}
                  class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-500/20 data-[state=checked]:bg-surface-500/10"
                >
                  <Menu.ItemText class="text-sm font-medium">{state}</Menu.ItemText>
                  <Menu.ItemIndicator>
                    <div class="h-1.5 w-1.5 rounded-full bg-current"></div>
                  </Menu.ItemIndicator>
                </Menu.OptionItem>
              {/each}
              {#if filteredStates.length === 0 && !stateSearch.toLowerCase().includes("all states")}
                <div class="px-4 py-8 text-center text-xs italic opacity-30">No states match search</div>
              {/if}
            </div>
          </Menu.Content>
        </Menu.Positioner>
      </Portal>
    </Menu>
  </div>

  <!-- Search Input -->
  <div class="relative flex h-10 flex-1 items-center md:col-span-2 lg:min-w-64 lg:flex-1">
    <Search class="pointer-events-none absolute top-1/2 left-3 z-10 size-4 -translate-y-1/2 text-surface-300" />
    <input
      type="text"
      bind:value={serviceRegex}
      placeholder="Process Regex ..."
      class="input h-10 w-full border-surface-500/30 bg-surface-50 dark:bg-surface-950/40 pl-9 backdrop-blur-sm"
    />
  </div>

  <!-- Bulk Action -->
  <button
    class="preset-filled-surface-500/20 hover:preset-filled-surface-500/40 btn flex items-center justify-center gap-2 border border-surface-500/20 backdrop-blur-sm md:col-span-2 lg:col-auto lg:ml-auto lg:shrink-0"
  >
    <ListFilter class="size-4" />
    Bulk Action
  </button>
</div>
