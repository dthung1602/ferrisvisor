<script lang="ts">
  import { Eye, EyeOff, Pencil, Server } from "@lucide/svelte";

  import type { Group } from "$lib/api/group";
  import type { Host, NewHost } from "$lib/api/host";
  import { formatDate } from "$lib/common";

  interface Props {
    host: Host | NewHost;
    groups: Group[];
    isEdit?: boolean;
    onSave: (e: Event) => void;
    onDiscard: () => void;
    onDelete?: () => void;
  }

  let { host = $bindable(), groups, isEdit = false, onSave, onDiscard, onDelete }: Props = $props();
  let showPassword = $state(false);

  function asHost(h: Host | NewHost): Host {
    return h as Host;
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
        <Server class="size-6 text-primary-500" />
      {/if}
      <h3 class="text-lg font-bold uppercase">{isEdit ? "Edit Host" : "Create Host"}</h3>
    </div>
    {#if isEdit}
      <p class="font-mono text-[10px] font-bold tracking-widest text-primary-500 uppercase opacity-50">
        Target: host #{asHost(host).id}
      </p>
    {/if}
  </div>

  <form class="space-y-8" onsubmit={onSave}>
    <div class="space-y-4">
      <div class="grid grid-cols-4 gap-4">
        <!-- Group Selection -->
        <div class="col-span-4 space-y-1.5">
          <label for="group_id" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">Group</label>
          <select
            class="select rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
            name="group_id"
            required
            bind:value={host.group_id}
          >
            <option value={0} disabled>Select a group</option>
            {#each groups as g (g.id)}
              <option value={g.id}>{g.name}</option>
            {/each}
          </select>
        </div>

        <div class="col-span-4 space-y-1.5">
          <label for="name" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">Display name</label>
          <input
            class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
            type="text"
            name="name"
            required
            bind:value={host.name}
          />
        </div>

        <div class="col-span-3 space-y-1.5">
          <label for="hostname" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50"
            >Hostname / IP</label
          >
          <input
            class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
            type="text"
            name="hostname"
            required
            bind:value={host.hostname}
          />
        </div>

        <div class="col-span-1 space-y-1.5">
          <label for="port" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">Port</label>
          <input
            class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
            type="number"
            name="port"
            required
            bind:value={host.port}
          />
        </div>

        <div class="col-span-2 space-y-1.5">
          <label for="username" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">Username</label>
          <input
            class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
            type="text"
            name="username"
            required
            bind:value={host.username}
          />
        </div>

        <div class="col-span-2 space-y-1.5">
          <label for="password" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">Password</label>
          <div class="relative">
            <input
              class="input w-full rounded-xl border-none bg-surface-500/10 px-4 py-3 pr-12 text-sm focus:ring-2 focus:ring-primary-500/20"
              type={showPassword ? "text" : "password"}
              name="password"
              required
              bind:value={host.password}
            />
            <button
              type="button"
              class="absolute top-1/2 right-4 -translate-y-1/2 text-surface-400 hover:text-primary-500"
              onclick={() => (showPassword = !showPassword)}
            >
              {#if showPassword}
                <EyeOff class="size-5" />
              {:else}
                <Eye class="size-5" />
              {/if}
            </button>
          </div>
        </div>

        {#if isEdit}
          <div class="col-span-2 space-y-1.5">
            <label for="created_at" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50"
              >Created at</label
            >
            <input
              class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
              type="text"
              name="created_at"
              disabled
              value={formatDate(asHost(host).created_at)}
            />
          </div>
          <div class="col-span-2 space-y-1.5">
            <label for="updated_at" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50"
              >Updated at</label
            >
            <input
              class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
              type="text"
              name="updated_at"
              disabled
              value={formatDate(asHost(host).updated_at)}
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
