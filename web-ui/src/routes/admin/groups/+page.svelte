<script lang="ts">
  import { Layers, LayersPlus } from "@lucide/svelte";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { api } from "$lib";
  import { clone } from "lodash";

  import type { Group } from "$lib/api/group";
  import { standardizeColor } from "$lib/common";
  import ConfirmationModal from "$lib/components/ConfirmationModal.svelte";
  import EntityList from "$lib/components/EntityList.svelte";
  import { toaster } from "$lib/toaster";

  import GroupForm from "./GroupForm.svelte";

  let groups: Group[] = $state([]);
  let selectedGroup: Group | null = $state(null);
  let selectedGroupCopy: Group | null = $state(null);
  let loading = $state(true);
  let deleteModalOpen = $state(false);

  async function selectGroup(group: Group | null) {
    selectedGroup = clone(group);
    selectedGroupCopy = clone(group);
  }

  async function fetchInitData() {
    try {
      groups = await api.group.list();
    } catch (e) {
      console.error("Failed to fetch groups", e);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    fetchInitData();
  });

  function handleCreateGroup() {
    goto(resolve("/admin/groups/new"));
  }

  async function handleSave(e: Event) {
    e.preventDefault();
    if (!selectedGroup) return;
    try {
      selectedGroup.color = standardizeColor(selectedGroup.color);
      await api.group.update(selectedGroup.id, selectedGroup);
      await fetchInitData();
      selectedGroupCopy = clone(selectedGroup);
      toaster.success({ title: "Group Saved", description: "The group has been updated successfully." });
    } catch (e) {
      console.error("Failed to save group", e);
      toaster.error({ title: "Save Failed", description: "Could not update the group. Please try again." });
    }
  }

  async function handleDeleteSelectedGroup() {
    if (!selectedGroup) return;
    deleteModalOpen = true;
  }

  async function confirmDeleteGroup() {
    if (!selectedGroup) return;
    try {
      await api.group.remove(selectedGroup.id);
      await selectGroup(null);
      await fetchInitData();
      toaster.success({ title: "Group Deleted", description: "The group has been removed successfully." });
    } catch (e) {
      console.error("Failed to delete group", e);
      toaster.error({ title: "Delete Failed", description: "Could not remove the group." });
    }
  }

  function resetSelectedGroup() {
    if (!selectedGroupCopy) return;
    selectedGroup = clone(selectedGroupCopy);
  }
</script>

<div class="space-y-8">
  <!-- Header Section -->
  <div class="flex flex-col gap-6 md:flex-row md:items-end md:justify-between">
    <div>
      <h2 class="text-4xl font-black tracking-tighter uppercase">Group Management</h2>
      <p class="mt-1 font-medium tracking-wide text-secondary-700-300">Group / environment registry</p>
    </div>
    <button
      onclick={handleCreateGroup}
      class="text-on-primary btn flex items-center gap-2 rounded-full bg-linear-to-br from-primary-500 to-tertiary-500 px-6 py-2.5 font-bold shadow-lg shadow-primary-500/20 transition-transform hover:scale-105 active:scale-95"
    >
      <LayersPlus class="size-4" />
      <span>Create group</span>
    </button>
  </div>

  <div class="grid grid-cols-12 gap-8">
    <!-- Groups Registry (Bento-style list) -->
    <div class="col-span-12 space-y-4 lg:col-span-7">
      <EntityList
        bind:selectedEntity={selectedGroup}
        entityName="group"
        avatarIconFunc={() => Layers}
        avatarColorFunc={(g) => g.color}
        entityNameFunc={(g) => g.name}
        searchFields={(g) => [g.name, g.id]}
        selectEntity={selectGroup}
        entities={groups}
        {loading}
      />
    </div>

    <!-- Edit View Panel -->
    <div class="col-span-12 lg:col-span-5">
      {#if selectedGroup}
        <GroupForm
          bind:group={selectedGroup}
          isEdit={true}
          onSave={handleSave}
          onDiscard={resetSelectedGroup}
          onDelete={handleDeleteSelectedGroup}
        />
      {:else}
        <div
          class="flex h-full flex-col items-center justify-center card rounded-xl border-2 border-dashed border-surface-200 dark:border-surface-800 bg-surface-50 dark:bg-surface-950/40 p-12 text-center opacity-70 shadow-sm"
        >
          <Layers class="mb-4 size-16" />
          <p class="text-sm font-bold tracking-[0.2em] uppercase">Select a group edit</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<ConfirmationModal
  bind:open={deleteModalOpen}
  title="Delete Group"
  message="Are you sure you want to delete this group? This action cannot be undone."
  confirmText="Delete Group"
  onConfirm={confirmDeleteGroup}
/>
