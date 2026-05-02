<script lang="ts">
  import { LayersPlus, Pencil } from "@lucide/svelte";

  import type { Group, NewGroup } from "$lib/api/group";
  import { formatDate } from "$lib/common";

  interface Props {
    group: Group | NewGroup;
    isEdit?: boolean;
    onSave: (e: Event) => void;
    onDiscard: () => void;
    onDelete?: () => void;
  }

  let { group = $bindable(), isEdit = false, onSave, onDiscard, onDelete }: Props = $props();

  function asGroup(g: Group | NewGroup): Group {
    return g as Group;
  }
</script>

<div
  class="relative overflow-hidden card rounded-xl border border-surface-500/10 bg-surface-50-950/40 p-8 shadow-2xl backdrop-blur-xl"
>
  <div class="mb-8">
    <div class="mb-2 flex items-center gap-3">
      {#if isEdit}
        <Pencil class="size-6 text-primary-500" />
      {:else}
        <LayersPlus class="size-6 text-primary-500" />
      {/if}
      <h3 class="text-lg font-bold uppercase">{isEdit ? "Edit Group" : "Create Group"}</h3>
    </div>
    {#if isEdit}
      <p class="opacity-50c font-mono text-[10px] font-bold tracking-widest text-primary-500 uppercase">
        Target: group #{asGroup(group).id}
      </p>
    {/if}
  </div>

  <form class="space-y-8" onsubmit={onSave}>
    <!-- Basic Config -->
    <div class="space-y-4">
      <div class="grid grid-cols-2 gap-4">
        <div class="col-span-2 space-y-1.5">
          <label for="name" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">Name</label>
          <input
            class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
            type="text"
            name="name"
            required
            bind:value={group.name}
          />
        </div>

        <div class="col-span-2 space-y-1.5">
          <label for="description" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">
            Description
          </label>
          <input
            class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
            type="text"
            name="description"
            required
            bind:value={group.description}
          />
        </div>

        <div class="col-span-2 space-y-1.5">
          <label for="color" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">Color</label>
          <div class="flex gap-4">
            <input
              class="input flex-1 rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
              type="text"
              name="color"
              placeholder="Any valid CSS color"
              required
              bind:value={group.color}
            />
            <div
              class="size-11 shrink-0 rounded-xl border border-surface-500/20"
              style="background-color: {group.color || '#000000'}"
            ></div>
          </div>
        </div>

        {#if isEdit}
          <div class="space-y-1.5">
            <label for="created_at" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50"
              >Created at</label
            >
            <input
              class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
              type="text"
              name="created_at"
              disabled
              value={formatDate(asGroup(group).created_at)}
            />
          </div>
          <div class="space-y-1.5">
            <label for="updated_at" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50"
              >Updated at</label
            >
            <input
              class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
              type="text"
              name="updated_at"
              disabled
              value={formatDate(asGroup(group).updated_at)}
            />
          </div>
        {/if}
      </div>
    </div>

    <!-- Actions -->
    <div class="flex items-center justify-between gap-8">
      <div>
        {#if isEdit && onDelete}
          <button
            class="btn preset-filled-error-500 px-6 py-3 font-bold text-error-contrast-500 transition-all hover:preset-filled-error-500 active:scale-95"
            type="button"
            onclick={onDelete}
          >
            Delete
          </button>
        {/if}
      </div>
      <div class="flex items-center justify-end gap-4">
        <button
          class="btn preset-outlined-surface-500 px-6 py-3 font-bold transition-all hover:preset-filled-surface-500 active:scale-95"
          type="button"
          onclick={onDiscard}
        >
          {isEdit ? "Discard" : "Cancel"}
        </button>
        <button
          class="text-on-primary btn min-w-32 preset-filled-primary-500 py-3 font-bold transition-all hover:shadow-lg hover:shadow-primary-500/20 active:scale-95"
          type="submit"
        >
          {isEdit ? "Save" : "Create"}
        </button>
      </div>
    </div>
  </form>
</div>
