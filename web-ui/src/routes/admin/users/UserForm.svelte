<script lang="ts">
  import { Switch } from "@skeletonlabs/skeleton-svelte";
  import { UserPen, UserPlus, X } from "@lucide/svelte";
  import type { User, NewUser } from "$lib/api/user";
  import type { Host } from "$lib/api/host";
  import type { Group } from "$lib/api/group";
  import { formatDate } from "$lib/string";

  type PermissionFormData = {
    id: number;
    user_id: number;
    group_id: number;
    host_id: number | null;
    service_name: string;
    can_view: boolean;
    can_act: boolean;
  };

  interface Props {
    user: User | NewUser;
    permissions: PermissionFormData[];
    toDeletePermissions?: Set<number>;
    groups: Group[];
    hosts: Host[];
    isEdit?: boolean;
    onSave: (e: Event) => void;
    onDiscard: () => void;
    onDelete?: () => void;
  }

  let {
    user = $bindable(),
    permissions = $bindable(),
    toDeletePermissions = $bindable(new Set()),
    groups,
    hosts,
    isEdit = false,
    onSave,
    onDiscard,
    onDelete
  }: Props = $props();

  // Counter for new permissions (negative IDs)
  let newPermCounter = $state(-1);

  function filterHostOfGroup(groupId: number) {
    return hosts.filter((host) => host.group_id === groupId);
  }

  function handleAddPerm() {
    let newPerm: PermissionFormData = {
      id: newPermCounter--,
      user_id: (user as User).id || 0,
      group_id: groups[0]?.id || 0,
      host_id: null,
      service_name: "",
      can_view: false,
      can_act: false
    };

    permissions = [newPerm, ...permissions];

    // Smooth scroll to top of permission list
    setTimeout(() => {
      document.getElementById("perm-list")?.scrollTo({ top: 0, behavior: "smooth" });
    }, 0);
  }

  function handleDeletePermission(id: number) {
    if (id > 0) {
      toDeletePermissions.add(id);
    }
    permissions = permissions.filter((perm) => perm.id !== id);
  }

  function asUser(u: User | NewUser): User {
    return u as User;
  }
</script>

<div
  class="relative overflow-hidden card rounded-xl border border-surface-500/10 bg-surface-50-950/40 p-8 shadow-2xl backdrop-blur-xl"
>
  <div class="mb-8">
    <div class="mb-2 flex items-center gap-3">
      {#if isEdit}
        <UserPen class="size-6 text-primary-500" />
      {:else}
        <UserPlus class="size-6 text-primary-500" />
      {/if}
      <h3 class="text-lg font-bold uppercase">{isEdit ? "Edit User" : "Create User"}</h3>
    </div>
    {#if isEdit}
      <p class="opacity-50c font-mono text-[10px] font-bold tracking-widest text-primary-500 uppercase">
        Target: user #{asUser(user).id}
      </p>
    {/if}
  </div>

  <form class="space-y-8" onsubmit={onSave}>
    <!-- Basic Config -->
    <div class="space-y-4">
      <div class="grid grid-cols-2 gap-4">
        <div class="space-y-1.5 {isEdit ? '' : 'col-span-2'}">
          <label for="email" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">Email</label>
          <input
            class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
            type="email"
            name="email"
            required
            bind:value={user.email}
          />
        </div>

        {#if !isEdit}
          <div class="col-span-2 space-y-1.5">
            <label for="password" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50"
              >Password</label
            >
            <input
              class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
              type="password"
              name="password"
              required
              bind:value={user.password}
            />
          </div>
        {/if}

        <div class="space-y-1.5 {isEdit ? '' : 'col-span-2'}">
          <label for="is_admin" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50"
            >Access Level</label
          >
          <select
            class="select rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
            name="is_admin"
            bind:value={user.is_admin}
          >
            <option value={true}>Administrator</option>
            <option value={false}>Operator</option>
          </select>
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
              value={formatDate(asUser(user).created_at)}
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
              value={formatDate(asUser(user).updated_at)}
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
              value={formatDate(asUser(user).last_login)}
            />
          </div>
        {/if}
      </div>
    </div>

    <!-- Host Permissions Section -->
    {#if !user.is_admin}
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
          {#each permissions as perm, i (perm.id)}
            <div
              class="space-y-3 rounded-xl border border-surface-500/5 bg-surface-500/5 p-4 transition-colors hover:bg-surface-500/10"
            >
              <div class="space-y-1 text-xs font-bold text-secondary-500">
                <span>RULE #{perm.id < 0 ? i : perm.id}</span>
                <button
                  type="button"
                  onclick={() => handleDeletePermission(perm.id)}
                  class="float-right ml-auto text-secondary-500 opacity-70 transition-all hover:text-error-500 hover:opacity-100"
                >
                  <X class="size-4" />
                </button>
              </div>
              <div class="space-y-1.5">
                <label for="group" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">Group</label>
                <select
                  class="select rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
                  bind:value={perm.group_id}
                >
                  {#each groups as group (group.id)}
                    <option value={group.id}>{group.name}</option>
                  {/each}
                </select>
              </div>
              <div class="space-y-1.5">
                <label for="host" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50">Host</label>
                <select
                  class="select rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
                  bind:value={perm.host_id}
                >
                  <option value={null}>-- All hosts --</option>
                  {#each filterHostOfGroup(perm.group_id) as host (host.id)}
                    <option value={host.id}>{host.name}</option>
                  {/each}
                </select>
              </div>
              <div class="space-y-1.5">
                <label for="service_name" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50"
                  >Service Name Pattern</label
                >
                <input
                  class="input rounded-xl border-none bg-surface-500/10 px-4 py-3 text-sm focus:ring-2 focus:ring-primary-500/20"
                  type="text"
                  name="service_name"
                  bind:value={perm.service_name}
                />
              </div>
              <label for="permission" class="ml-1 text-[10px] font-bold tracking-widest uppercase opacity-50"
                >Permission</label
              >
              <div class="flex items-center gap-6 pt-2">
                <Switch checked={perm.can_view} onCheckedChange={(details) => (perm.can_view = details.checked)}>
                  <Switch.Control>
                    <Switch.Thumb />
                  </Switch.Control>
                  <Switch.Label>Can view</Switch.Label>
                  <Switch.HiddenInput />
                </Switch>
                <Switch
                  checked={perm.can_act}
                  onCheckedChange={(details) => {
                    perm.can_act = details.checked;
                    perm.can_view ||= details.checked; // can act implies can view
                  }}
                >
                  <Switch.Control>
                    <Switch.Thumb />
                  </Switch.Control>
                  <Switch.Label>Can act</Switch.Label>
                  <Switch.HiddenInput />
                </Switch>
              </div>
            </div>
          {/each}

          {#if permissions.length === 0}
            <div class="py-8 text-center text-xs italic opacity-30">No active rules found for this user</div>
          {/if}
        </div>
      </div>
    {/if}

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
