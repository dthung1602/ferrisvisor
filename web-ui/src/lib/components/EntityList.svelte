<script lang="ts" generics="E extends { id: number }">
  import type { Snippet } from "svelte";
  import { Search } from "@lucide/svelte";
  import type { LucideIcon } from "@lucide/svelte";
  import type { User } from "$lib/api/user";

  type Prop = {
    selectedEntity: E | null;
    entityName: string;
    avatarFunc: (e: E) => LucideIcon;
    entityNameFunc: (e: E) => string;
    badge?: Snippet<[User]>;
    selectEntity: (e: E | null) => Promise<void>;
    entities: E[];
    loading: boolean;
  };

  let {
    selectedEntity = $bindable(null),
    entityName,
    avatarFunc,
    entityNameFunc,
    badge,
    selectEntity,
    entities,
    loading
  }: Prop = $props();

  let searchTerm = $state("");

  let filteredEntities = $derived.by(() => {
    const normalizedSearchTerm = searchTerm.toLowerCase().trim();
    if (!normalizedSearchTerm) return entities;
    return entities.filter((e) => entityNameFunc(e).toLowerCase().includes(normalizedSearchTerm) || e.id.toString().includes(normalizedSearchTerm));
  });
</script>

<div
  class="relative overflow-hidden card rounded-xl border border-surface-500/10 bg-surface-50-950/40 p-6 backdrop-blur-xl"
>
  <div class="mb-6 flex items-center justify-between">
    <h3 class="flex flex-1 items-center gap-2 text-lg font-bold uppercase">
      <span class="size-2 animate-pulse rounded-full bg-secondary-500"></span>
      ACTIVE {entityName}
    </h3>
    <input
      class="input max-w-sm flex-1 rounded-xl border-none bg-surface-500/10 px-4 py-2 text-sm focus:ring-2 focus:ring-primary-500/20"
      placeholder="Search..."
      bind:value={searchTerm}
    />
  </div>

  {#if loading}
    <div class="flex h-64 items-center justify-center">
      <div class="size-12 animate-spin rounded-full border-4 border-surface-500/10 border-t-primary-500"></div>
    </div>
  {:else}
    <div class="space-y-3">
      {#each filteredEntities as entity (entity.id)}
        {@const AvatarIcon = avatarFunc(entity)}

        <div
          class="group flex cursor-pointer items-center justify-between card border-l-4 p-4 transition-all
                {selectedEntity?.id === entity.id
            ? 'border-primary-500 bg-primary-500/10 hover:bg-primary-500/15'
            : 'border-surface-500/20 bg-surface-500/5 hover:bg-surface-500/10'}"
          onclick={() => selectEntity(entity)}
          onkeydown={(e) => e.key === "Enter" && selectEntity(entity)}
          role="button"
          tabindex="0"
        >
          <div class="flex items-center gap-4">
            <div class="flex size-12 items-center justify-center rounded-lg bg-surface-500/10">
              <AvatarIcon class="size-6 text-secondary-500" />
            </div>
            <div class="overflow-hidden">
              <div class="flex items-center gap-2">
                <span class="truncate font-bold tracking-tight">{entityNameFunc(entity)}</span>
                {#if badge}
                  {@render badge(entity)}
                {/if}
              </div>
              <div class="truncate font-mono text-xs opacity-50">
                ID: {entity.id}
              </div>
            </div>
          </div>
        </div>
      {/each}

      {#if entities.length === 0}
        <div class="flex flex-col items-center justify-center py-12 text-center opacity-30">
          <Search class="mb-4 size-12" />
          <p class="font-bold tracking-widest uppercase">No operators found in registry</p>
        </div>
      {/if}
    </div>
  {/if}
</div>
