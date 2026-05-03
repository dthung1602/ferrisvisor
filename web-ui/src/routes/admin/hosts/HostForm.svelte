<script lang="ts">
  import { ChevronDown, Eye, EyeOff, Pencil, Search, Server } from "@lucide/svelte";
  import { Menu, Portal } from "@skeletonlabs/skeleton-svelte";

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
  let groupMenuOpen = $state(false);
  let groupSearch = $state("");

  let filteredGroups = $derived(groups.filter((g) => g.name.toLowerCase().includes(groupSearch.toLowerCase())));

  let selectedGroupName = $derived(groups.find((g) => g.id === host.group_id)?.name || "Select a group");

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
          <Menu
            open={groupMenuOpen}
            onOpenChange={(e) => {
              groupMenuOpen = e.open;
              if (!e.open) groupSearch = "";
            }}
            positioning={{ placement: "bottom-start", gutter: 8, sameWidth: true }}
          >
            <Menu.Trigger
              class="flex w-full items-center justify-between rounded-xl border-none bg-surface-500/10 px-4 py-3 text-left text-sm transition-all hover:bg-surface-500/20 active:scale-[0.99]"
            >
              <span class="truncate {host.group_id === 0 ? 'text-surface-500/50' : ''}">
                {selectedGroupName}
              </span>
              <ChevronDown class="size-4 shrink-0 transition-transform {groupMenuOpen ? 'rotate-180' : ''}" />
            </Menu.Trigger>
            <Portal>
              <Menu.Positioner>
                <Menu.Content
                  class="z-100 overflow-hidden rounded-xl border border-surface-500 bg-surface-100-900 shadow-2xl backdrop-blur-xl"
                >
                  <div class="border-b border-surface-500/10 p-2">
                    <div class="relative">
                      <Search class="pointer-events-none absolute top-1/2 left-3 size-3 -translate-y-1/2 opacity-40" />
                      <input
                        type="text"
                        bind:value={groupSearch}
                        placeholder="Filter groups..."
                        class="w-full rounded-lg bg-surface-500/10 py-1.5 pr-3 pl-8 text-xs focus:ring-2 focus:ring-primary-500/50 focus:outline-none"
                        onclick={(e) => e.stopPropagation()}
                      />
                    </div>
                  </div>

                  <div class="max-h-64 overflow-y-auto">
                    {#each filteredGroups as g (g.id)}
                      <Menu.OptionItem
                        type="radio"
                        value={g.id.toString()}
                        checked={host.group_id === g.id}
                        onCheckedChange={(checked) => {
                          if (checked) host.group_id = g.id;
                        }}
                        class="flex w-full items-center justify-between gap-3 px-4 py-3 text-left transition-colors hover:bg-surface-500/20 data-[state=checked]:bg-surface-500/10"
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
