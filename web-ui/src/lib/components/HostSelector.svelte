<script lang="ts">
  import { ChevronDown, Search } from "@lucide/svelte";
  import { Menu, Portal } from "@skeletonlabs/skeleton-svelte";

  type Prop = {
    hosts: { id: number; name: string }[];
    selectedHostId: number | null;
    setSelectedHostId: (hostId: number | null) => void;
  };

  let { hosts, selectedHostId, setSelectedHostId }: Prop = $props();

  let selectedHostName = $derived(
    selectedHostId === null ? "All Hosts" : hosts.find((h) => h.id === selectedHostId)?.name || "Unknown"
  );

  let hostMenuOpen = $state(false);
  let hostSearch = $state("");

  let filteredHosts = $derived(hosts.filter((h) => h.name.toLowerCase().includes(hostSearch.toLowerCase())));
</script>

<Menu
  open={hostMenuOpen}
  onOpenChange={(e) => {
    hostMenuOpen = e.open;
    if (!e.open) hostSearch = "";
  }}
  positioning={{ placement: "bottom-start", gutter: 8, sameWidth: true }}
>
  <Menu.Trigger
    class="flex w-full items-center justify-between rounded-xl border-2 border-surface-200 bg-white
     px-4 py-3 text-left text-sm transition-all hover:bg-surface-100/20 active:scale-[0.99]
     dark:border-surface-700 dark:bg-surface-900 dark:hover:bg-surface-500/40"
  >
    <span class="truncate {selectedHostId === null ? 'text-surface-500/50' : ''}">
      {selectedHostName}
    </span>
    <ChevronDown class="size-4 shrink-0 transition-transform {hostMenuOpen ? 'rotate-180' : ''}" />
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
              bind:value={hostSearch}
              placeholder="Filter hosts..."
              class="w-full rounded-xl border border-surface-200 bg-white py-1.5 pr-3 pl-8 text-xs shadow-inner focus:ring-2 focus:ring-primary-500/50 focus:outline-none dark:border-surface-700 dark:bg-surface-900"
              onclick={(e) => e.stopPropagation()}
            />
          </div>
        </div>

        <div class="max-h-96 overflow-y-auto">
          {#if "all hosts".includes(hostSearch.toLowerCase())}
            <Menu.OptionItem
              type="radio"
              value="null"
              checked={selectedHostId === null}
              onCheckedChange={() => {
                setSelectedHostId(null);
                hostSearch = "";
              }}
              class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-100/20 data-[state=checked]:bg-surface-500/10 dark:hover:bg-surface-500/40"
            >
              <Menu.ItemText class="text-sm font-medium">All Hosts</Menu.ItemText>
              <Menu.ItemIndicator>
                <div class="h-1.5 w-1.5 rounded-full bg-current"></div>
              </Menu.ItemIndicator>
            </Menu.OptionItem>
          {/if}
          {#each filteredHosts as host (host.id)}
            <Menu.OptionItem
              type="radio"
              value={host.id.toString()}
              checked={selectedHostId === host.id}
              onCheckedChange={(checked) => {
                if (checked) {
                  setSelectedHostId(host.id);
                  hostSearch = "";
                }
              }}
              class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-100/20 data-[state=checked]:bg-surface-500/10 dark:hover:bg-surface-500/40"
            >
              <Menu.ItemText class="text-sm font-medium">{host.name}</Menu.ItemText>
              <Menu.ItemIndicator>
                <div class="h-1.5 w-1.5 rounded-full bg-current"></div>
              </Menu.ItemIndicator>
            </Menu.OptionItem>
          {/each}
          {#if filteredHosts.length === 0 && !hostSearch.toLowerCase().includes("all hosts")}
            <div class="px-4 py-8 text-center text-xs italic opacity-30">No hosts match search</div>
          {/if}
        </div>
      </Menu.Content>
    </Menu.Positioner>
  </Portal>
</Menu>
