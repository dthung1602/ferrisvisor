<script lang="ts">
  import { Group as GroupIcon } from "@lucide/svelte";

  import type { Group } from "$lib/api/group";
  import GroupSelector from "$lib/components/GroupSelector.svelte";

  type Props = {
    selectedGroupId: number | null;
    groups: Group[];
  };
  let { selectedGroupId = $bindable(null), groups }: Props = $props();

  let selectedGroupColor: string = $derived.by(() => {
    let group = groups.find((g) => g.id === selectedGroupId);
    if (!group) {
      return "surface-500";
    }
    return group.color;
  });
</script>

<div
  class="group relative z-50 flex min-h-35 flex-col justify-between overflow-visible rounded-xl border border-(--group-color)/30
            bg-(--group-color)/15 p-5 transition-all"
  style="--group-color: {selectedGroupColor}"
>
  <div class="mb-2 flex items-start justify-between">
    <span class="text-xs font-bold tracking-widest uppercase">Group</span>
    <GroupIcon class="size-5" />
  </div>

  <div class="relative">
    <GroupSelector
      {groups}
      {selectedGroupId}
      setSelectedGroupId={(id) => (selectedGroupId = id)}
      showSearchBox={false}
    />
  </div>
</div>
