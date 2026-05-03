<script lang="ts">
  import { TriangleAlert, X } from "@lucide/svelte";
  import { Dialog, Portal } from "@skeletonlabs/skeleton-svelte";

  type Props = {
    open: boolean;
    title: string;
    message: string;
    confirmText?: string;
    cancelText?: string;
    onConfirm: () => void;
    onCancel?: () => void;
  };

  let {
    open = $bindable(),
    title,
    message,
    confirmText = "Confirm",
    cancelText = "Cancel",
    onConfirm,
    onCancel
  }: Props = $props();

  function handleConfirm() {
    onConfirm();
    open = false;
  }

  function handleCancel() {
    if (onCancel) onCancel();
    open = false;
  }
</script>

<Dialog {open} onOpenChange={(e) => (open = e.open)}>
  <Portal>
    <Dialog.Backdrop class="fixed inset-0 z-100 bg-black/60 backdrop-blur-sm" />
    <Dialog.Positioner class="fixed inset-0 z-110 flex items-center justify-center p-4">
      <Dialog.Content
        class="w-full max-w-sm overflow-hidden rounded-xl border border-surface-500/20 bg-surface-100-900 shadow-2xl"
      >
        <div class="flex items-center justify-between border-b border-surface-500/10 px-6 py-4">
          <div class="flex items-center gap-3">
            <TriangleAlert class="size-5 text-error-500" />
            <Dialog.Title class="text-lg font-bold tracking-tight">{title}</Dialog.Title>
          </div>
          <Dialog.CloseTrigger class="rounded-full p-1 hover:bg-surface-500/10">
            <X class="size-5" />
          </Dialog.CloseTrigger>
        </div>

        <div class="p-6">
          <Dialog.Description class="text-sm opacity-70">
            {message}
          </Dialog.Description>
        </div>

        <div class="flex justify-end gap-3 bg-surface-500/5 px-6 py-4">
          <button class="preset-tonal-surface-500 btn" onclick={handleCancel}>
            {cancelText}
          </button>
          <button class="btn preset-filled-error-500 font-bold" onclick={handleConfirm}>
            {confirmText}
          </button>
        </div>
      </Dialog.Content>
    </Dialog.Positioner>
  </Portal>
</Dialog>
