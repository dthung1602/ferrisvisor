<script lang="ts">
  import { Search, ShieldUser, User, UserCheck, UserPen, UserPlus, X } from "lucide-svelte";
  import { api } from "$lib";
  import type { User as UserType } from "$lib/api/user";
  import type { Permission } from "$lib/api/permission";
  import type { Host } from "$lib/api/host";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";
  import { clone, omit } from "lodash";

  type PermissionFormData = {
    id: number;
    user_id: number;
    host_id: number;
    service_name: string;
    can_view: boolean;
    can_act: boolean;
  };

  let searchTerm = $state("");
  let users: UserType[] = $state([]);
  let hosts: Host[] = $state([]);
  let selectedUser: UserType | null = $state(null);
  let selectedUserCopy: UserType | null = $state(null);
  let selectedUserPerms: PermissionFormData[] = $state([]);
  let toDeletePermissions: Set<number> = $state(new Set());
  let loading = $state(true);

  async function fetchInitData() {
    const userRes = api.user.list().then(userData => {
      users = userData;
      if (users.length > 0 && !selectedUser) {
        selectUser(users[0]);
      } else {
        selectUser(selectedUser ? (users.find(u => u.id === selectedUser?.id) ?? null) : null)
      }
    })
    const hostRes = api.host.list().then(hostData => {
      hosts = hostData;
    })
    return Promise.all([userRes, hostRes]).catch(console.error).finally(() => loading = false);
  }

  async function fetchPermission(user: UserType) {
    try {
      let rawPermData: Permission[] = await api.permission.list(user.id);
      selectedUserPerms = rawPermData.map((perm) => ({
        id: perm.id,
        user_id: perm.user_id,
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
    if (user) {
      await fetchPermission(user);
    }
  }

  function resetSelectedUser() {
    if (!selectedUserCopy) return;
    selectedUser = clone(selectedUserCopy);
    fetchPermission(selectedUser);
  }

  onMount(fetchInitData);

  let filteredUsers = $derived.by(() => {
    let normalizedSearchTerm = searchTerm.toLowerCase().trim();
    if (!normalizedSearchTerm) return users;
    return users.filter((user) => user.email.toLowerCase().includes(normalizedSearchTerm));
  });

  function handleCreateOperator() {
    goto(resolve("/admin/users/new"));
  }

  // Counter to generate unique IDs for new permissions
  // Negative IDs are used to mark permissions that should be created
  let newPermCounter = $state(-1);
  function handleAddPerm() {
    let newPerm: PermissionFormData = {
      id: newPermCounter--,
      user_id: selectedUser?.id as number,
      host_id: 0,
      service_name: "",
      can_view: false,
      can_act: false
    };

    selectedUserPerms = [newPerm, ...selectedUserPerms];

    document.getElementById("perm-list")?.scrollTo({ top: 0, behavior: "smooth" });
  }

  function handleDeletePermission(id: number) {
    if (id < 0) return; // remove not-yet-created permissions
    toDeletePermissions.add(id);
    selectedUserPerms = selectedUserPerms.filter((perm) => perm.id !== id);
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
    const uid = selectedUser.id as number;

    const updateUserResult = api.user.update(uid, selectedUser);

    const updatePermissionResults = [];
    const newPermissionResults = [];
    for (let perm of selectedUserPerms) {
      if (perm.id > 0) {
        updatePermissionResults.push(
          api.permission.update(uid, perm.id as number, omit(perm, "id"))
        )
      } else {
        newPermissionResults.push(
          api.permission.create(uid, omit(perm, "id"))
        )
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

  function formatDate(date: string) {
    // TODO timezone
    return new Date(date).toLocaleString();
  }
</script>

<div class="space-y-8">
  <!-- Header Section -->
  <div class="flex flex-col gap-6 md:flex-row md:items-end md:justify-between">
    <div>
      <h2 class="text-4xl font-black tracking-tighter uppercase">User Management</h2>
      <p class="mt-1 font-medium tracking-wide text-secondary-500">Command & Control / Identity Registry</p>
    </div>
    <button
      onclick={handleCreateOperator}
      class="text-on-primary btn flex items-center gap-2 rounded-full bg-linear-to-br from-primary-500 to-tertiary-500 px-6 py-2.5 font-bold shadow-lg shadow-primary-500/20 transition-transform hover:scale-105 active:scale-95"
    >
      <UserPlus class="size-4" />
      <span>Create user</span>
    </button>
  </div>

  <div class="grid grid-cols-12 gap-8">
    <!-- Users Registry (Bento-style list) -->
    <div class="col-span-12 space-y-4 lg:col-span-7">
      <div
        class="relative overflow-hidden card rounded-xl border border-surface-500/10 bg-surface-50-950/40 p-6 backdrop-blur-xl"
      >
        <div class="mb-6 flex items-center justify-between">
          <h3 class="flex flex-1 items-center gap-2 text-lg font-bold">
            <span class="size-2 animate-pulse rounded-full bg-secondary-500"></span>
            ACTIVE USER
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
            {#each filteredUsers as user (user.id)}
              <div
                class="group flex cursor-pointer items-center justify-between card border-l-4 p-4 transition-all hover:bg-surface-500/10
                {selectedUser?.id === user.id
                  ? 'border-primary-500 bg-primary-500/10'
                  : 'border-surface-500/20 bg-surface-500/5'}"
                onclick={() => selectUser(user)}
                onkeydown={(e) => e.key === "Enter" && selectUser(user)}
                role="button"
                tabindex="0"
              >
                <div class="flex items-center gap-4">
                  <div class="flex size-12 items-center justify-center rounded-lg bg-surface-500/10">
                    {#if user.is_admin}
                      <ShieldUser class="size-6 text-secondary-500" />
                    {:else}
                      <User class="size-6 opacity-40" />
                    {/if}
                  </div>
                  <div class="overflow-hidden">
                    <div class="flex items-center gap-2">
                      <span class="truncate font-bold tracking-tight">{user.email}</span>
                      {#if user.is_admin}
                        <span
                          class="rounded bg-secondary-500/20 px-2 py-0.5 text-[9px] font-bold tracking-widest text-secondary-500 uppercase"
                          >Admin</span
                        >
                      {:else}
                        <span
                          class="rounded bg-surface-500/20 px-2 py-0.5 text-[9px] font-bold tracking-widest text-surface-500 uppercase"
                          >Operator</span
                        >
                      {/if}
                    </div>
                    <div class="truncate font-mono text-xs opacity-50">
                      ID: {user.id}
                    </div>
                  </div>
                </div>
              </div>
            {/each}

            {#if users.length === 0}
              <div class="flex flex-col items-center justify-center py-12 text-center opacity-30">
                <Search class="mb-4 size-12" />
                <p class="font-bold tracking-widest uppercase">No operators found in registry</p>
              </div>
            {/if}
          </div>
        {/if}
      </div>
    </div>

    <!-- Edit View Panel -->
    <div class="col-span-12 lg:col-span-5">
      {#if selectedUser}
        <div
          class="relative overflow-hidden card rounded-xl border border-surface-500/10 bg-surface-50-950/40 p-8 shadow-2xl backdrop-blur-xl"
        >
          <div class="mb-8">
            <div class="mb-2 flex items-center gap-3">
              <UserPen class="size-6 text-primary-500" />
              <h3 class="text-lg font-bold">EDIT USER</h3>
            </div>
            <p class="opacity-50c font-mono text-[10px] font-bold tracking-widest text-primary-500 uppercase">
              Target: user #{selectedUser.id}
            </p>
          </div>

          <form class="space-y-8" onsubmit={handleSave}>
            <!-- Basic Config -->
            <div class="space-y-4">
              <div class="grid grid-cols-2 gap-4">
                <div class="space-y-1.5">
                  <label for="email" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50"
                    >Email</label
                  >
                  <input
                    class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
                    type="text"
                    name="email"
                    bind:value={selectedUser.email}
                  />
                </div>
                <div class="space-y-1.5">
                  <label for="is_admin" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50"
                    >Access Level</label
                  >
                  <select
                    class="select rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
                    name="is_admin"
                    bind:value={selectedUser.is_admin}
                  >
                    <option value={true}>Administrator</option>
                    <option value={false}>Operator</option>
                  </select>
                </div>
                <div class="space-y-1.5">
                  <label for="created_at" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50"
                    >Created at</label
                  >
                  <input
                    class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
                    type="text"
                    name="created_at"
                    disabled
                    value={formatDate(selectedUser.created_at)}
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
                    value={formatDate(selectedUser.updated_at)}
                  />
                </div>
                <div class="space-y-1.5">
                  <label for="last_login" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50"
                    >Last login</label
                  >
                  <input
                    class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
                    type="text"
                    name="last_login"
                    disabled
                    value={formatDate(selectedUser.last_login)}
                  />
                </div>
              </div>
            </div>

            <!-- Host Permissions Section -->
            {#if !selectedUser.is_admin}
              <div class="space-y-4 border-t border-surface-500/10 pt-6">
                <div class="flex items-center justify-between">
                  <h4 class="font-black text-primary-500 uppercase">Host Permissions</h4>
                  <button
                    type="button"
                    onclick={handleAddPerm}
                    class="text-[12px] font-bold tracking-widest text-secondary-500 uppercase hover:underline"
                  >
                    + Add Rule
                  </button>
                </div>

                <div id="perm-list" class="max-h-120 space-y-3 overflow-y-auto pr-2">
                  {#each selectedUserPerms as perm (perm.id)}
                    <div
                      class="space-y-3 rounded-xl border border-surface-500/5 bg-surface-500/5 p-4 transition-colors hover:bg-surface-500/10"
                    >
                      <div class="space-y-1.5">
                        <label for="host" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">
                          Service Name Pattern
                        </label>
                        <select
                          class="select rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
                          bind:value={perm.host_id}
                        >
                          {#each hosts as host (host.id)}
                            <option value={host.id}>{host.name}</option>
                          {/each}
                        </select>
                      </div>
                      <div class="space-y-1.5">
                        <label for="service_name" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">
                          Service Name Pattern
                        </label>
                        <input
                          class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
                          type="text"
                          name="service_name"
                          bind:value={perm.service_name}
                        />
                      </div>
                      <div class="flex items-center gap-6">
                        <label class="group flex cursor-pointer items-center gap-2">
                          <input type="checkbox" bind:checked={perm.can_view} class="checkbox" />
                          <span class="text-xs font-medium opacity-60 group-hover:opacity-100">Can view</span>
                        </label>
                        <label class="group flex cursor-pointer items-center gap-2">
                          <input type="checkbox" bind:checked={perm.can_act} class="checkbox" />
                          <span class="text-xs font-medium opacity-60 group-hover:opacity-100">Can act</span>
                        </label>
                        <button
                          type="button"
                          onclick={() => handleDeletePermission(perm.id)}
                          class="ml-auto opacity-70 transition-all hover:text-error-500 hover:opacity-100"
                        >
                          <X class="size-4" />
                        </button>
                      </div>
                    </div>
                  {/each}

                  {#if selectedUserPerms.length === 0}
                    <div class="py-8 text-center text-xs italic opacity-30">No active rules found for this operator</div>
                  {/if}
                </div>
              </div>
            {/if}

            <!-- Actions -->
            <div class="flex items-center justify-between gap-8 pt-8">
              <div>
                <button
                  class="btn text-error-contrast-500 preset-filled-error-500 px-6 py-3 font-bold transition-all hover:preset-filled-error-500 active:scale-95"
                  type="button"
                  onclick={handleDeleteSelectedUser}
                >
                  Delete
                </button>
              </div>
              <div class="flex items-center justify-end gap-4">
                <button
                  class="btn preset-outlined-surface-500 px-6 py-3 font-bold transition-all hover:preset-filled-surface-500 active:scale-95"
                  type="button"
                  onclick={resetSelectedUser}
                >
                  Discard
                </button>
                <button
                  class="btn text-on-primary preset-filled-primary-500 min-w-32  py-3 font-bold transition-all hover:shadow-lg hover:shadow-primary-500/20 active:scale-95"
                  type="submit"
                >
                  Save
                </button>
              </div>
            </div>
          </form>
        </div>
      {:else}
        <div
          class="flex h-full flex-col items-center justify-center card rounded-xl border-2 border-dashed border-surface-500/10 p-12 text-center opacity-30"
        >
          <UserCheck class="mb-4 size-16" />
          <p class="text-sm font-bold tracking-[0.2em] uppercase">Select an operator to initialize command console</p>
        </div>
      {/if}
    </div>
  </div>
</div>
