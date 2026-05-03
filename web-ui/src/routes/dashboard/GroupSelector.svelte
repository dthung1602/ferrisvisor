<script lang="ts">
  import { ChevronDown, Group as GroupIcon } from "@lucide/svelte";
  import { Menu, Portal } from "@skeletonlabs/skeleton-svelte";

  import type { Group } from "$lib/api/group";

  type Props = {
    selectedGroupId: number | null;
    groups: Group[];
  };
  let { selectedGroupId = $bindable(null), groups }: Props = $props();

  let selectedGroup: Group | null = $derived(groups.find((g) => g.id === selectedGroupId) ?? null);
  let groupDropdownOpen: boolean = $state(false);
</script>

<div
  class="group relative z-50 flex min-h-35 flex-col justify-between overflow-visible rounded-xl border border-(--group-color)/30
            bg-(--group-color)/15 p-5 transition-all"
  style="--group-color: {selectedGroup?.color || 'surface-500'}"
>
  <div class="mb-2 flex items-start justify-between">
    <span class="text-xs font-bold tracking-widest uppercase">Group</span>
    <GroupIcon class="size-5" />
  </div>

  <div class="relative">
    <Menu
      open={groupDropdownOpen}
      onOpenChange={(e) => (groupDropdownOpen = e.open)}
      positioning={{ placement: "bottom-start", gutter: 8, sameWidth: true }}
    >
      <Menu.Trigger
        class="flex w-full items-center justify-between rounded-lg bg-black/10 px-4 py-3 transition-all hover:bg-black/20 active:scale-[0.98]"
      >
        <span class="text-lg font-black tracking-tight text-surface-900-100">
          {selectedGroup?.name || "Select Group"}
        </span>
        <ChevronDown class="size-4 text-surface-900-100 transition-transform {groupDropdownOpen ? 'rotate-180' : ''}" />
      </Menu.Trigger>

      <Portal>
        <Menu.Positioner>
          <Menu.Content
            class="z-100 overflow-hidden rounded-xl border border-surface-500 bg-surface-100-900 shadow-2xl backdrop-blur-xl"
          >
            {#each groups as group (group.id)}
              <Menu.OptionItem
                type="radio"
                value={group.id.toString()}
                checked={selectedGroupId === group.id}
                onCheckedChange={(checked) => {
                  if (checked) selectedGroupId = group.id;
                }}
                class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-500/20 data-[state=checked]:bg-surface-300/20"
              >
                <div class="flex items-center gap-3">
                  <span class="h-2 w-2 rounded-full" style="background-color: {group.color}"></span>
                  <Menu.ItemText class="text-sm font-medium">{group.name}</Menu.ItemText>
                </div>
              </Menu.OptionItem>
            {/each}
          </Menu.Content>
        </Menu.Positioner>
      </Portal>
    </Menu>
  </div>
</div>
