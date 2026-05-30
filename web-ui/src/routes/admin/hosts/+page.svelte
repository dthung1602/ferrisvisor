<script lang="ts">
  import { Server, ServerCog } from "@lucide/svelte";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { api } from "$lib";
  import { clone } from "lodash";

  import type { Group } from "$lib/api/group";
  import type { Host } from "$lib/api/host";
  import ConfirmationModal from "$lib/components/ConfirmationModal.svelte";
  import EntityList from "$lib/components/EntityList.svelte";
  import { toaster } from "$lib/toaster";

  import HostForm from "./HostForm.svelte";

  let hosts: Host[] = $state([]);
  let groups: Group[] = $state([]);
  let selectedHost: Host | null = $state(null);
  let selectedHostCopy: Host | null = $state(null);
  let loading = $state(true);
  let deleteModalOpen = $state(false);

  async function selectHost(host: Host | null) {
    selectedHost = clone(host);
    selectedHostCopy = clone(host);
  }

  async function fetchInitData() {
    try {
      const [hostList, groupList] = await Promise.all([api.host.list(null), api.group.list()]);
      hosts = hostList;
      groups = groupList;
    } catch (e) {
      console.error("Failed to fetch init data", e);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    fetchInitData();
  });

  function handleCreateHost() {
    goto(resolve("/admin/hosts/new"));
  }

  async function handleSave(e: Event) {
    e.preventDefault();
    if (!selectedHost) return;
    try {
      await api.host.update(selectedHost.id, selectedHost);
      await fetchInitData();
      toaster.success({ title: "Host Saved", description: "The host has been updated successfully." });
    } catch (e) {
      console.error("Failed to save host", e);
      toaster.error({ title: "Save Failed", description: "Could not update the host. Please try again." });
    }
  }

  async function handleDeleteSelectedHost() {
    if (!selectedHost) return;
    deleteModalOpen = true;
  }

  async function confirmDeleteHost() {
    if (!selectedHost) return;
    try {
      await api.host.remove(selectedHost.id);
      await selectHost(null);
      await fetchInitData();
      toaster.success({ title: "Host Deleted", description: "The host has been removed successfully." });
    } catch (e) {
      console.error("Failed to delete host", e);
      toaster.error({ title: "Delete Failed", description: "Could not remove the host." });
    }
  }

  function resetSelectedHost() {
    if (!selectedHostCopy) return;
    selectedHost = clone(selectedHostCopy);
  }
</script>

<div class="space-y-8">
  <!-- Header Section -->
  <div class="flex flex-col gap-6 md:flex-row md:items-end md:justify-between">
    <div>
      <h2 class="text-4xl font-black tracking-tighter uppercase">Host Management</h2>
      <p class="mt-1 font-medium tracking-wide text-secondary-700-300">Host / server inventory</p>
    </div>
    <button
      onclick={handleCreateHost}
      class="text-on-primary btn flex items-center gap-2 rounded-full bg-linear-to-br from-primary-500 to-tertiary-500 px-6 py-2.5 font-bold shadow-lg shadow-primary-500/20 transition-transform hover:scale-105 active:scale-95"
    >
      <ServerCog class="size-4" />
      <span>Create host</span>
    </button>
  </div>

  <div class="grid grid-cols-12 gap-8">
    <!-- Hosts Registry (Bento-style list) -->
    <div class="col-span-12 space-y-4 lg:col-span-7">
      <EntityList
        bind:selectedEntity={selectedHost}
        entityName="host"
        avatarIconFunc={() => Server}
        entityNameFunc={(h) => h.name}
        searchFields={(h) => [h.name, h.hostname, h.username, h.id]}
        selectEntity={selectHost}
        entities={hosts}
        {loading}
      >
        {#snippet badge(host)}
          {@const group = groups.find((g) => g.id === host.group_id)}
          <span
            class="rounded px-2 py-0.5 text-[9px] font-bold tracking-widest uppercase
            {group?.color ? '' : 'bg-secondary-500/20 text-secondary-700-300'}"
            style={group?.color ? `background-color: ${group.color}33; color: ${group.color}` : ""}
          >
            {group?.name ?? "Unknown"}
          </span>
        {/snippet}
      </EntityList>
    </div>

    <!-- Edit View Panel -->
    <div class="col-span-12 lg:col-span-5">
      {#if selectedHost}
        <HostForm
          bind:host={selectedHost}
          {groups}
          isEdit={true}
          onSave={handleSave}
          onDiscard={resetSelectedHost}
          onDelete={handleDeleteSelectedHost}
        />
      {:else}
        <div
          class="flex h-full flex-col items-center justify-center card rounded-xl border-2 border-dashed border-surface-200 bg-surface-50 p-12 text-center opacity-70 shadow-sm dark:border-surface-800 dark:bg-surface-950/40"
        >
          <Server class="mb-4 size-16" />
          <p class="text-sm font-bold tracking-[0.2em] uppercase">Select a host to edit</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<ConfirmationModal
  bind:open={deleteModalOpen}
  title="Delete Host"
  message="Are you sure you want to delete this host? This action cannot be undone."
  confirmText="Delete Host"
  onConfirm={confirmDeleteHost}
/>
