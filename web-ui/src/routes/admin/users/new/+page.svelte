<script lang="ts">
  import { api } from "$lib";
  import type { Host } from "$lib/api/host";
  import type { Group } from "$lib/api/group";
  import UserForm from "../UserForm.svelte";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";
  import { omit } from "lodash";

  type PermissionFormData = {
    id: number;
    user_id: number;
    group_id: number;
    host_id: number | null;
    service_name: string;
    can_view: boolean;
    can_act: boolean;
  };

  let formData = $state({
    email: "",
    password: "",
    is_admin: false
  });

  let permissions: PermissionFormData[] = $state([]);
  let hosts: Host[] = $state([]);
  let groups: Group[] = $state([]);

  async function fetchInitData() {
    const hostRes = api.host.list().then(hostData => {
      hosts = hostData;
    })
    const groupRes = api.group.list().then(groupData => {
      groups = groupData;
    })
    return Promise.all([hostRes, groupRes]).catch(console.error);
  }

  onMount(fetchInitData);

  async function handleSave(e: Event) {
    e.preventDefault();
    try {
      const newUser = await api.user.create(formData);
      const uid = newUser.id;

      const newPermissionResults = permissions.map(perm => 
        api.permission.create(uid, omit(perm, "id"))
      );

      await Promise.all(newPermissionResults);
      await goto(resolve("/admin/users"));
    } catch (e) {
      console.error(e);
      alert("Failed to create user. Please try again.");
    }
  }

  function handleCancel() {
    goto(resolve("/admin/users"));
  }
</script>

<div class="mx-auto max-w-2xl space-y-8 py-12 px-4">
  <UserForm
    bind:user={formData}
    bind:permissions={permissions}
    groups={groups}
    hosts={hosts}
    isEdit={false}
    onSave={handleSave}
    onDiscard={handleCancel}
  />
</div>
