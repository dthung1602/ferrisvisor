<script lang="ts">
  import { ChevronDown, Search } from "@lucide/svelte";
  import { Menu, Portal } from "@skeletonlabs/skeleton-svelte";

  import { PROCESS_STATES, type ProcessState } from "$lib/constants";

  type Prop = {
    selectedProcessState: ProcessState | null;
    setSelectedProcessState: (state: ProcessState | null) => void;
  };

  let { selectedProcessState, setSelectedProcessState }: Prop = $props();

  let stateMenuOpen = $state(false);
  let stateSearch = $state("");

  let filteredStates = $derived(PROCESS_STATES.filter((s) => s.toLowerCase().includes(stateSearch.toLowerCase())));
</script>

<Menu
  open={stateMenuOpen}
  onOpenChange={(e) => {
    stateMenuOpen = e.open;
    if (!e.open) stateSearch = "";
  }}
  positioning={{ placement: "bottom-start", gutter: 8, sameWidth: true }}
>
  <Menu.Trigger
    class="flex w-full items-center justify-between rounded-xl border-2 border-surface-200 bg-white
     px-4 py-3 text-left text-sm transition-all hover:bg-surface-100/20 active:scale-[0.99]
     dark:border-surface-700 dark:bg-surface-900 dark:hover:bg-surface-500/40"
  >
    <span class="truncate {selectedProcessState === null ? 'text-surface-800-200' : ''}">
      {selectedProcessState || "All States"}
    </span>
    <ChevronDown class="size-4 shrink-0 transition-transform {stateMenuOpen ? 'rotate-180' : ''}" />
  </Menu.Trigger>
  <Portal>
    <Menu.Positioner>
      <Menu.Content
        class="z-100 overflow-hidden rounded-xl border-2 border-surface-200 bg-white shadow-xl backdrop-blur-xl dark:border-surface-700 dark:bg-surface-900"
      >
        <div class="p-2">
          <div class="relative">
            <Search class="pointer-events-none absolute top-1/2 left-3 size-3 -translate-y-1/2 opacity-40" />
            <input
              type="text"
              bind:value={stateSearch}
              placeholder="Filter states..."
              class="w-full rounded-xl border border-surface-200 bg-white py-1.5 pr-3 pl-8 text-xs shadow-inner focus:ring-2 focus:ring-primary-500/50 focus:outline-none dark:border-surface-700 dark:bg-surface-900"
              onclick={(e) => e.stopPropagation()}
            />
          </div>
        </div>

        <div class="max-h-118 overflow-y-auto">
          {#if "all states".includes(stateSearch.toLowerCase())}
            <Menu.OptionItem
              type="radio"
              value="null"
              checked={selectedProcessState === null}
              onCheckedChange={() => {
                setSelectedProcessState(null);
                stateSearch = "";
              }}
              class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-100/20 data-[state=checked]:bg-surface-500/10 dark:hover:bg-surface-500/40"
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
                if (checked) {
                  setSelectedProcessState(state);
                  stateSearch = "";
                }
              }}
              class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-100/20 data-[state=checked]:bg-surface-500/10 dark:hover:bg-surface-500/40"
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
