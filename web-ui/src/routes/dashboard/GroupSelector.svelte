<script lang="ts">
  import { ChevronDown, Group as GroupIcon } from "@lucide/svelte";

  import type { Group } from "$lib/api/group";

  type Props = {
    selectedGroupId: number | null;
    groups: Group[];
  };
  let { selectedGroupId = $bindable(null), groups }: Props = $props();

  let selectedGroup: Group | null = $state(null);
  let groupDropdownOpen: boolean = $state(false);

  $effect(() => {
    selectedGroup = groups.find((g) => g.id === selectedGroupId) ?? null;
    groupDropdownOpen = false;
  });
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
    <button
      onclick={() => (groupDropdownOpen = !groupDropdownOpen)}
      class="flex w-full items-center justify-between rounded-lg bg-black/10 px-4 py-3 transition-all hover:bg-black/20 active:scale-[0.98]"
    >
      <span class="text-lg font-black tracking-tight text-surface-900-100">
        {selectedGroup?.name || "Select Group"}
      </span>
      <ChevronDown class="size-4 text-surface-900-100 transition-transform {groupDropdownOpen ? 'rotate-180' : ''}" />
    </button>

    {#if groupDropdownOpen}
      <div
        class="absolute top-full left-0 z-100 mt-2 w-full overflow-hidden rounded-xl border border-surface-500 bg-surface-100-900 shadow-2xl backdrop-blur-xl"
      >
        {#each groups as group (group.id)}
          <button
            onclick={() => (selectedGroupId = group.id)}
            class="flex w-full items-center gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-500/20"
          >
            <span class="h-2 w-2 rounded-full" style="background-color: {group.color}"></span>
            <span class="text-sm">{group.name}</span>
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>
