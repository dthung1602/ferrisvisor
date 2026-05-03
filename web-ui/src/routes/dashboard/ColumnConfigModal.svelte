<script lang="ts">
  import { GripVertical, Lock, X } from "@lucide/svelte";
  import { Dialog, Portal } from "@skeletonlabs/skeleton-svelte";

  import type { ProcessColumn } from "./common.ts";

  let { open = $bindable(), columns = $bindable() }: { open: boolean; columns: ProcessColumn[] } = $props();

  let draggedIndex = $state<number | null>(null);
  let dropTargetIndex = $state<number | null>(null);

  function handleDragStart(index: number) {
    if (columns[index].locked) return;
    draggedIndex = index;
  }

  function handleDragOver(e: DragEvent, index: number) {
    e.preventDefault();
    if (draggedIndex === null || columns[index].locked) return;
    dropTargetIndex = index;
  }

  function handleDrop(index: number) {
    if (draggedIndex === null || draggedIndex === index || columns[index].locked) {
      draggedIndex = null;
      dropTargetIndex = null;
      return;
    }

    const newColumns = [...columns];
    const [draggedItem] = newColumns.splice(draggedIndex, 1);
    newColumns.splice(index, 0, draggedItem);
    columns = newColumns;

    draggedIndex = null;
    dropTargetIndex = null;
  }

  function handleDragEnd() {
    draggedIndex = null;
    dropTargetIndex = null;
  }
</script>

<Dialog {open} onOpenChange={(e) => (open = e.open)}>
  <Portal>
    <Dialog.Backdrop class="fixed inset-0 z-100 bg-black/60 backdrop-blur-sm" />
    <Dialog.Positioner class="fixed inset-0 z-100 flex items-center justify-center p-4">
      <Dialog.Content
        class="w-full max-w-md overflow-hidden rounded-xl border border-surface-500/20 bg-surface-100-900 shadow-2xl"
      >
        <div class="flex items-center justify-between border-b border-surface-500/10 px-6 py-4">
          <Dialog.Title class="text-lg font-bold tracking-tight">Configure Columns</Dialog.Title>
          <Dialog.CloseTrigger class="rounded-full p-1 hover:bg-surface-500/10">
            <X class="size-5" />
          </Dialog.CloseTrigger>
        </div>

        <div class="space-y-4 p-6">
          <div class="space-y-2" role="list">
            {#each columns as col, i (col.id)}
              <div
                role="listitem"
                draggable={!col.locked}
                ondragstart={() => handleDragStart(i)}
                ondragover={(e) => handleDragOver(e, i)}
                ondrop={() => handleDrop(i)}
                ondragend={handleDragEnd}
                class="flex items-center justify-between rounded-lg border border-surface-500/10 bg-surface-500/5 p-3 transition-all
                {col.locked ? 'cursor-not-allowed opacity-50' : 'cursor-grab active:cursor-grabbing'}
                {draggedIndex === i ? 'opacity-20' : ''}
                {dropTargetIndex === i ? 'border-primary-500 bg-primary-500/10' : ''}"
              >
                <div class="flex items-center gap-3">
                  {#if col.locked}
                    <Lock size="16" class="text-surface-500" />
                  {:else}
                    <GripVertical size="16" class="text-surface-500" />
                  {/if}
                  <label class="flex items-center gap-3 {col.locked ? 'cursor-not-allowed' : 'cursor-pointer'}">
                    <input type="checkbox" bind:checked={col.visible} disabled={col.locked} class="checkbox" />
                    <span class="text-sm font-medium">{col.label}</span>
                  </label>
                </div>
              </div>
            {/each}
          </div>
        </div>
      </Dialog.Content>
    </Dialog.Positioner>
  </Portal>
</Dialog>
