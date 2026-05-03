<script lang="ts" generics="E extends { id: number }">
  import type { Snippet } from "svelte";

  import { Search } from "@lucide/svelte";
  import type { LucideIcon } from "@lucide/svelte";

  type Prop = {
    selectedEntity: E | null;
    entityName: string;
    avatarIconFunc: (e: E) => LucideIcon;
    avatarColorFunc?: (e: E) => string;
    entityNameFunc: (e: E) => string;
    searchFields?: (e: E) => (string | number | undefined | null)[];
    badge?: Snippet<[E]>;
    selectEntity: (e: E | null) => Promise<void>;
    entities: E[];
    loading: boolean;
  };

  let {
    selectedEntity = $bindable(null),
    entityName,
    avatarIconFunc,
    avatarColorFunc,
    entityNameFunc,
    searchFields,
    badge,
    selectEntity,
    entities,
    loading
  }: Prop = $props();

  let searchTerm = $state("");

  let filteredEntities = $derived.by(() => {
    const term = searchTerm.toLowerCase().trim();
    if (!term) return entities;

    return entities.filter((e) => {
      const fields = searchFields ? searchFields(e) : [entityNameFunc(e), e.id];

      return fields.some((f) => f?.toString().toLowerCase().includes(term));
    });
  });
</script>

<div
  class="relative overflow-hidden card rounded-xl border border-surface-500/10 bg-surface-50-950/40 p-6 backdrop-blur-xl"
>
  <div class="mb-6 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
    <h3 class="flex items-center gap-2 text-lg font-bold uppercase">
      <span class="size-2 animate-pulse rounded-full bg-secondary-500"></span>
      ACTIVE {entityName}
    </h3>
    <div class="relative max-w-sm flex-1">
      <Search class="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 opacity-40" />
      <input
        class="w-full rounded-xl border-none bg-surface-500/10 py-2 pr-4 pl-10 text-sm focus:ring-2 focus:ring-primary-500/50 focus:outline-none"
        placeholder="Search..."
        bind:value={searchTerm}
      />
    </div>
  </div>

  {#if loading}
    <div class="flex h-64 items-center justify-center">
      <div class="size-12 animate-spin rounded-full border-4 border-surface-500/10 border-t-primary-500"></div>
    </div>
  {:else}
    <div class="space-y-3">
      {#each filteredEntities as entity (entity.id)}
        {@const AvatarIcon = avatarIconFunc(entity)}
        {@const avatarColor = avatarColorFunc?.(entity)}

        <div
          class="group flex cursor-pointer items-center justify-between card border-r-2 p-4 transition-all
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
              <AvatarIcon class="size-6 {avatarColor ? '' : 'text-secondary-500'}" color={avatarColor ?? undefined} />
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

      {#if filteredEntities.length === 0}
        <div class="flex flex-col items-center justify-center py-16 text-center opacity-30">
          <Search class="mb-4 size-16" />
          <p class="text-sm font-bold tracking-[0.2em] uppercase">No {entityName} found in registry</p>
        </div>
      {/if}
    </div>
  {/if}
</div>
