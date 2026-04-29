<script lang="ts">
  import { onMount } from "svelte";

  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { api } from "$lib";
  import { omit } from "lodash";

  import type { Group } from "$lib/api/group";
  import type { Host } from "$lib/api/host";

  import UserForm from "../UserForm.svelte";

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
    const hostRes = api.host.list(null).then((hostData) => {
      hosts = hostData;
    });
    const groupRes = api.group.list().then((groupData) => {
      groups = groupData;
    });
    return Promise.all([hostRes, groupRes]).catch(console.error);
  }

  onMount(fetchInitData);

  async function handleSave(e: Event) {
    e.preventDefault();
    try {
      const newUser = await api.user.create(formData);
      const uid = newUser.id;

      const newPermissionResults = permissions.map((perm) => api.permission.create(uid, omit(perm, "id")));

      await Promise.all(newPermissionResults);
      await goto(resolve("/admin/users"));
    } catch (e) {
      console.error(e);
      alert("Failed to create user. Please try again.");
    }
  }

  function handleDiscard() {
    goto(resolve("/admin/users"));
  }
</script>

<div class="mx-auto max-w-2xl space-y-8 px-4 py-12">
  <UserForm
    bind:user={formData}
    bind:permissions
    {groups}
    {hosts}
    isEdit={false}
    onSave={handleSave}
    onDiscard={handleDiscard}
  />
</div>
