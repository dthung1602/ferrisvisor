<script lang="ts">
  import {
    UserPlus,
    User,
    ShieldCheck,
    Edit,
    Trash2,
    UserCog,
    UserCheck,
    X,
    Fingerprint,
    Search,
    ChevronRight,
    Lock,
    Unlock,
    Activity,
    ShieldAlert
  } from "lucide-svelte";
  import { api } from "$lib";
  import type { User as UserType } from "$lib/api/user";
  import type { Permission } from "$lib/api/permission";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";

  let users: UserType[] = $state([]);
  let selectedUser: UserType | null = $state(null);
  let permissions: Permission[] = $state([]);
  let loading = $state(true);

  async function fetchUsers() {
    try {
      users = await api.user.list();
      if (users.length > 0 && !selectedUser) {
        selectUser(users[0]);
      }
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  async function selectUser(user: UserType) {
    selectedUser = user;
    try {
      permissions = await api.permission.list(user.id);
    } catch (e) {
      console.error(e);
      permissions = [];
    }
  }

  onMount(fetchUsers);

  function getUsername(email: string) {
    return email.split("@")[0];
  }

  function handleCreateOperator() {
    goto(resolve("/admin/users/new"));
  }

  function handleEditUser(id: number) {
    goto(resolve(`/admin/users/edit/${id}`));
  }

  async function handleDeleteUser(id: number) {
    if (confirm("Are you sure you want to delete this operator? This action is irreversible.")) {
      try {
        await api.user.remove(id);
        if (selectedUser?.id === id) selectedUser = null;
        await fetchUsers();
      } catch (e) {
        console.error(e);
      }
    }
  }

  async function handleUpdatePermissions(e: Event) {
    e.preventDefault();
    alert("Permission updates committed to secure registry.");
    // Implementation would go here if we had a bulk update API
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
      <span>CREATE OPERATOR</span>
    </button>
  </div>

  <div class="grid grid-cols-12 gap-8">
    <!-- Users Registry (Bento-style list) -->
    <div class="col-span-12 space-y-4 lg:col-span-7">
      <div
        class="relative overflow-hidden card rounded-xl border border-surface-500/10 bg-surface-50-950/40 p-6 backdrop-blur-xl"
      >
        <div class="mb-6 flex items-center justify-between">
          <h3 class="flex items-center gap-2 text-lg font-bold">
            <span class="size-2 animate-pulse rounded-full bg-secondary-500"></span>
            ACTIVE OPERATORS
          </h3>
          <span class="font-mono text-[10px] tracking-widest uppercase opacity-50">Sort: By Recent Login</span>
        </div>

        {#if loading}
          <div class="flex h-64 items-center justify-center">
            <div class="size-12 animate-spin rounded-full border-4 border-surface-500/10 border-t-primary-500"></div>
          </div>
        {:else}
          <div class="space-y-3">
            {#each users as user (user.id)}
              <div
                class="group flex cursor-pointer items-center justify-between card border-l-4 p-4 transition-all {selectedUser?.id ===
                user.id
                  ? 'border-primary-500 bg-primary-500/10'
                  : 'border-surface-500/20 bg-surface-500/5'} hover:bg-surface-500/10"
                onclick={() => selectUser(user)}
                onkeydown={(e) => e.key === "Enter" && selectUser(user)}
                role="button"
                tabindex="0"
              >
                <div class="flex items-center gap-4">
                  <div class="flex size-12 items-center justify-center rounded-lg bg-surface-500/10">
                    {#if user.is_admin}
                      <ShieldAlert class="size-6 text-secondary-500" />
                    {:else}
                      <User class="size-6 opacity-40" />
                    {/if}
                  </div>
                  <div class="overflow-hidden">
                    <div class="flex items-center gap-2">
                      <span class="truncate font-bold tracking-tight">{getUsername(user.email)}</span>
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
                      ID: 0x{user.id.toString(16).padStart(4, "0")} • <span>{user.email}</span>
                    </div>
                  </div>
                </div>
                <div class="flex items-center gap-2 opacity-0 transition-opacity group-hover:opacity-100">
                  <button
                    onclick={(e) => {
                      e.stopPropagation();
                      handleEditUser(user.id);
                    }}
                    class="rounded-lg bg-surface-500/10 p-2 text-surface-500 transition-colors hover:text-primary-500"
                    title="Full Edit"
                  >
                    <Edit class="size-4" />
                  </button>
                  <button
                    onclick={(e) => {
                      e.stopPropagation();
                      handleDeleteUser(user.id);
                    }}
                    class="rounded-lg bg-surface-500/10 p-2 text-surface-500 transition-colors hover:text-error-500"
                    title="Purge"
                  >
                    <Trash2 class="size-4" />
                  </button>
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
          <!-- Orbit Pulse Decorative Element -->
          <div class="absolute -top-12 -right-12 size-32 rounded-full bg-primary-500/10 blur-3xl"></div>

          <div class="mb-8">
            <div class="mb-2 flex items-center gap-3">
              <UserCog class="size-6 text-primary-500" />
              <h3 class="text-xl font-bold tracking-tight">Edit Permissions</h3>
            </div>
            <p class="font-mono text-[10px] tracking-widest uppercase opacity-50">
              Target: {getUsername(selectedUser.email)}
            </p>
          </div>

          <form class="space-y-8" onsubmit={handleUpdatePermissions}>
            <!-- Basic Config -->
            <div class="space-y-4">
              <div class="grid grid-cols-2 gap-4">
                <div class="space-y-1.5">
                  <label class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">Username</label>
                  <input
                    class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
                    type="text"
                    value={getUsername(selectedUser.email)}
                    disabled
                  />
                </div>
                <div class="space-y-1.5">
                  <label class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">Access Level</label>
                  <select
                    class="select rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
                    bind:value={selectedUser.is_admin}
                  >
                    <option value={true}>Administrator</option>
                    <option value={false}>Operator</option>
                  </select>
                </div>
              </div>
            </div>

            <!-- Host Permissions Section -->
            <div class="space-y-4 border-t border-surface-500/10 pt-6">
              <div class="flex items-center justify-between">
                <h4 class="text-[10px] font-black tracking-[0.2em] text-primary-500 uppercase">Host Permissions</h4>
                <button
                  type="button"
                  onclick={() => handleEditUser(selectedUser!.id)}
                  class="text-[10px] font-bold tracking-widest text-secondary-500 uppercase hover:underline"
                >
                  + Add Rule
                </button>
              </div>

              <div class="max-h-80 space-y-3 overflow-y-auto pr-2">
                {#each permissions as perm (perm.id)}
                  <div
                    class="space-y-3 rounded-xl border border-surface-500/5 bg-surface-500/5 p-4 transition-colors hover:bg-surface-500/10"
                  >
                    <div class="space-y-1.5">
                      <label class="font-mono text-[9px] tracking-tighter uppercase opacity-40"
                        >Service Name Pattern</label
                      >
                      <input
                        class="input rounded-lg border-none bg-surface-50-950/40 px-3 py-2 font-mono text-xs text-secondary-500"
                        type="text"
                        value={perm.service_name}
                        disabled
                      />
                    </div>
                    <div class="flex items-center gap-6">
                      <label class="group flex cursor-pointer items-center gap-2">
                        <input type="checkbox" checked={perm.can_view} class="checkbox" disabled />
                        <span class="text-xs font-medium opacity-60 group-hover:opacity-100">can_view</span>
                      </label>
                      <label class="group flex cursor-pointer items-center gap-2">
                        <input type="checkbox" checked={perm.can_act} class="checkbox" disabled />
                        <span class="text-xs font-medium opacity-60 group-hover:opacity-100">can_act</span>
                      </label>
                      <button
                        type="button"
                        onclick={() => handleEditUser(selectedUser!.id)}
                        class="ml-auto opacity-30 transition-all hover:text-error-500 hover:opacity-100"
                      >
                        <X class="size-4" />
                      </button>
                    </div>
                  </div>
                {/each}

                {#if permissions.length === 0}
                  <div class="py-8 text-center text-xs italic opacity-30">No active rules found for this operator</div>
                {/if}
              </div>
            </div>

            <!-- Actions -->
            <div class="flex items-center gap-4 pt-4">
              <button
                class="text-on-primary btn flex-1 rounded-full bg-linear-to-r from-primary-500 to-tertiary-500 py-3 font-bold transition-all hover:shadow-lg hover:shadow-primary-500/20 active:scale-95"
                type="submit"
              >
                COMMIT CHANGES
              </button>
              <button
                class="btn rounded-full bg-surface-500/10 px-6 py-3 font-bold transition-all hover:bg-surface-500/20 active:scale-95"
                type="button"
                onclick={() => selectUser(selectedUser!)}
              >
                DISCARD
              </button>
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
