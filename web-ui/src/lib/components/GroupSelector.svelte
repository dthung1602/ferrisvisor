<script lang="ts">
  import { ChevronDown } from "@lucide/svelte";
  import { Menu, Portal } from "@skeletonlabs/skeleton-svelte";

  import type { Group } from "$lib/api/group";

  type Props = {
    groups: Group[];
    selectedGroupId: number | null;
    setSelectedGroupId: (groupId: number) => void;
  };

  let { groups, selectedGroupId = null, setSelectedGroupId }: Props = $props();

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
    class="flex w-full items-center justify-between rounded-xl border-2 border-surface-200 dark:border-surface-700
     bg-white dark:bg-surface-900 px-4 py-3 text-left text-sm transition-all
     hover:bg-surface-100/20 dark:hover:bg-surface-500/40 active:scale-[0.99]"
  >
    <span class="truncate {selectedGroupId === 0 ? 'text-surface-500/50' : ''}">
      {selectedGroupName}
    </span>
    <ChevronDown class="size-4 shrink-0 transition-transform {groupMenuOpen ? 'rotate-180' : ''}" />
  </Menu.Trigger>
  <Portal>
    <Menu.Positioner>
      <Menu.Content
        class="z-100 overflow-hidden rounded-xl border-2 border-surface-200 dark:border-surface-700 bg-white dark:bg-surface-900 shadow-xl backdrop-blur-xl"
      >
        <div class="max-h-64 overflow-y-auto">
          {#each filteredGroups as g (g.id)}
            <Menu.OptionItem
              type="radio"
              value={g.id.toString()}
              checked={selectedGroupId === g.id}
              onCheckedChange={(checked) => {
                if (checked) setSelectedGroupId(g.id);
              }}
              class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-100/20 dark:hover:bg-surface-500/40 data-[state=checked]:bg-surface-500/10"
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
