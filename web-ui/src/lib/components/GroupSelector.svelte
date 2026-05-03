<script lang="ts">
  import { ChevronDown, Search } from "@lucide/svelte";
  import { Menu, Portal } from "@skeletonlabs/skeleton-svelte";

  import type { Group } from "$lib/api/group";

  type Props = {
    groups: Group[];
    selectedGroupId: number | null;
    setSelectedGroupId: (groupId: number) => void;
    showSearchBox: boolean;
  };

  let { groups, selectedGroupId = null, setSelectedGroupId, showSearchBox = false }: Props = $props();

  let groupMenuOpen = $state(false);
  let groupSearch = $state("");

  let filteredGroups = $derived(groups.filter((g) => g.name.toLowerCase().includes(groupSearch.toLowerCase())));
  let selectedGroupName = $derived(groups.find((g) => g.id === selectedGroupId)?.name || "Select a group");
</script>

<Menu
  open={groupMenuOpen}
  onOpenChange={(e) => {
    groupMenuOpen = e.open;
    if (!e.open) groupSearch = "";
  }}
  positioning={{ placement: "bottom-start", gutter: 8, sameWidth: true }}
>
  <Menu.Trigger
    class="flex w-full items-center justify-between rounded-xl border-none bg-surface-500/10 px-4 py-3 text-left text-sm transition-all hover:bg-surface-500/20 active:scale-[0.99]"
  >
    <span class="truncate {selectedGroupId === 0 ? 'text-surface-500/50' : ''}">
      {selectedGroupName}
    </span>
    <ChevronDown class="size-4 shrink-0 transition-transform {groupMenuOpen ? 'rotate-180' : ''}" />
  </Menu.Trigger>
  <Portal>
    <Menu.Positioner>
      <Menu.Content
        class="z-100 overflow-hidden rounded-xl border border-surface-500 bg-surface-100-900 shadow-2xl backdrop-blur-xl"
      >
        {#if showSearchBox}
          <div class="border-b border-surface-500/10 p-2">
            <div class="relative">
              <Search class="pointer-events-none absolute top-1/2 left-3 size-3 -translate-y-1/2 opacity-40" />
              <input
                type="text"
                bind:value={groupSearch}
                placeholder="Filter groups..."
                class="w-full rounded-lg bg-surface-500/10 py-1.5 pr-3 pl-8 text-xs ring-0 outline-none focus:border-primary-500 focus:ring-1 focus:ring-primary-500 focus:outline-none"
                onclick={(e) => e.stopPropagation()}
              />
            </div>
          </div>
        {/if}

        <div class="max-h-64 overflow-y-auto">
          {#each filteredGroups as g (g.id)}
            <Menu.OptionItem
              type="radio"
              value={g.id.toString()}
              checked={selectedGroupId === g.id}
              onCheckedChange={(checked) => {
                if (checked) setSelectedGroupId(g.id);
              }}
              class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-500/20 data-[state=checked]:bg-surface-500/10"
            >
              <div class="flex items-center gap-3">
                <span class="h-2 w-2 rounded-full" style="background-color: {g.color}"></span>
                <Menu.ItemText class="text-sm font-medium">{g.name}</Menu.ItemText>
              </div>
            </Menu.OptionItem>
          {/each}
          {#if filteredGroups.length === 0}
            <div class="px-4 py-8 text-center text-xs italic opacity-30">No groups match search</div>
          {/if}
        </div>
      </Menu.Content>
    </Menu.Positioner>
  </Portal>
</Menu>
