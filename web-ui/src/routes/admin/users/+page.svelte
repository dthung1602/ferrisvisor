<script lang="ts">
  import { ShieldUser, User, UserCheck, UserPlus } from "@lucide/svelte";
  import { api } from "$lib";
  import type { User as UserType } from "$lib/api/user";
  import type { Permission } from "$lib/api/permission";
  import type { Host } from "$lib/api/host";
  import type { Group } from "$lib/api/group";
  import UserForm from "./UserForm.svelte";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { clone, omit } from "lodash";
  import EntityList from "$lib/components/EntityList.svelte";

  type PermissionFormData = {
    id: number;
    user_id: number;
    group_id: number;
    host_id: number | null;
    service_name: string;
    can_view: boolean;
    can_act: boolean;
  };

  let users: UserType[] = $state([]);
  let hosts: Host[] = $state([]);
  let groups: Group[] = $state([]);
  let selectedUser: UserType | null = $state(null);
  let selectedUserCopy: UserType | null = $state(null);
  let selectedUserPerms: PermissionFormData[] = $state([]);
  let toDeletePermissions: Set<number> = $state(new Set());
  let loading = $state(true);

  async function fetchInitData() {
    const userRes = api.user.list().then((userData) => {
      users = userData;
      if (users.length > 0 && !selectedUser) {
        selectUser(users[0]);
      } else {
        selectUser(selectedUser ? (users.find((u) => u.id === selectedUser?.id) ?? null) : null);
      }
    });
    const hostRes = api.host.list().then((hostData) => {
      hosts = hostData;
    });
    const groupRes = api.group.list().then((groupData) => {
      groups = groupData;
    });
    await Promise.all([userRes, hostRes, groupRes])
      .catch(console.error)
      .finally(() => (loading = false));
  }

  async function fetchPermission(user: UserType) {
    try {
      let rawPermData: Permission[] = await api.permission.list(user.id);
      selectedUserPerms = rawPermData.map((perm) => ({
        id: perm.id,
        user_id: perm.user_id,
        group_id: perm.group_id,
        host_id: perm.host_id,
        service_name: perm.service_name,
        can_view: perm.can_view,
        can_act: perm.can_act
      }));
    } catch (e) {
      console.error(e);
      selectedUserPerms = [];
    }
  }

  async function selectUser(user: UserType | null) {
    selectedUser = clone(user);
    selectedUserCopy = clone(user);
    toDeletePermissions = new Set();
    if (user) {
      await fetchPermission(user);
    }
  }

  function resetSelectedUser() {
    if (!selectedUserCopy) return;
    selectedUser = clone(selectedUserCopy);
    toDeletePermissions = new Set();
    fetchPermission(selectedUser);
  }

  $effect(() => {
    fetchInitData()
  });

  function handleCreateUser() {
    goto(resolve("/admin/users/new"));
  }

  async function handleDeleteSelectedUser() {
    if (!selectedUser) return;
    if (confirm(`Are you sure you want to delete user ${selectedUser.email}? This action is irreversible.`)) {
      try {
        await api.user.remove(selectedUser.id);
        await selectUser(null);
        await fetchInitData();
      } catch (e) {
        console.error(e);
      }
    }
  }

  async function handleSave(e: Event) {
    e.preventDefault();
    if (!selectedUser) return;
    const uid = selectedUser.id;

    const updateUserResult = api.user.update(uid, selectedUser);

    const updatePermissionResults = [];
    const newPermissionResults = [];
    for (let perm of selectedUserPerms) {
      if (perm.id > 0) {
        updatePermissionResults.push(api.permission.update(uid, perm.id as number, omit(perm, "id")));
      } else {
        newPermissionResults.push(api.permission.create(uid, omit(perm, "id")));
      }
    }

    let deletePermissionResults = Array.from(toDeletePermissions).map((id) => api.permission.remove(uid, id));

    try {
      await updateUserResult;
      // TODO make this bulk update
      await Promise.all(updatePermissionResults);
      await Promise.all(newPermissionResults);
      await Promise.all(deletePermissionResults);
      await fetchInitData();
      alert("User updated successfully!");
    } catch (e) {
      console.error(e);
      alert("Failed to update user. Please try again.");
    }
  }
</script>

<div class="space-y-8">
  <!-- Header Section -->
  <div class="flex flex-col gap-6 md:flex-row md:items-end md:justify-between">
    <div>
      <h2 class="text-4xl font-black tracking-tighter uppercase">User Management</h2>
      <p class="mt-1 font-medium tracking-wide text-secondary-500">Identity Registry</p>
    </div>
    <button
      onclick={handleCreateUser}
      class="text-on-primary btn flex items-center gap-2 rounded-full bg-linear-to-br from-primary-500 to-tertiary-500 px-6 py-2.5 font-bold shadow-lg shadow-primary-500/20 transition-transform hover:scale-105 active:scale-95"
    >
      <UserPlus class="size-4" />
      <span>Create user</span>
    </button>
  </div>

  <div class="grid grid-cols-12 gap-8">
    <!-- Users Registry (Bento-style list) -->
    <div class="col-span-12 space-y-4 lg:col-span-7">
      <EntityList
        bind:selectedEntity={selectedUser}
        entityName="user"
        avatarFunc={(u) => (u.is_admin ? ShieldUser : User)}
        entityNameFunc={(u) => u.email}
        selectEntity={selectUser}
        entities={users}
        {loading}
      >
        {#snippet badge(user)}
          {#if user.is_admin}
            <span
              class="rounded bg-secondary-500/20 px-2 py-0.5 text-[9px] font-bold tracking-widest text-secondary-500 uppercase"
            >
              Admin
            </span>
          {:else}
            <span
              class="rounded bg-surface-500/20 px-2 py-0.5 text-[9px] font-bold tracking-widest text-surface-500 uppercase"
            >
              Operator
            </span>
          {/if}
        {/snippet}
      </EntityList>
    </div>

    <!-- Edit View Panel -->
    <div class="col-span-12 lg:col-span-5">
      {#if selectedUser}
        <UserForm
          bind:user={selectedUser}
          bind:permissions={selectedUserPerms}
          bind:toDeletePermissions
          {groups}
          {hosts}
          isEdit={true}
          onSave={handleSave}
          onDiscard={resetSelectedUser}
          onDelete={handleDeleteSelectedUser}
        />
      {:else}
        <div
          class="flex h-full flex-col items-center justify-center card rounded-xl border-2 border-dashed border-surface-500/10 p-12 text-center opacity-30"
        >
          <UserCheck class="mb-4 size-16" />
          <p class="text-sm font-bold tracking-[0.2em] uppercase">Select user to edit</p>
        </div>
      {/if}
    </div>
  </div>
</div>
