<script lang="ts">
  import { page } from "$app/state";
  import { api } from "$lib";
  import type { Permission } from "$lib/api/permission";
  import EntityForm from "$lib/components/EntityForm.svelte";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { pick } from "lodash";
  import type { Host } from "$lib/api/host";

  type NewPermission = Omit<Permission, "id" | "user_id" | "host_name" | "group_id" | "group_name">;

  const userId = parseInt(page.params.id || "0");
  const permId = parseInt(page.params.permId || "0");

  async function onSubmit(formData: NewPermission) {
    try {
      await api.permission.update(userId, permId, formData);
      await goto(resolve(`/admin/users/edit/${userId}`));
    } catch (e) {
      console.error(e);
      error = e + "";
    }
  }

  let formData: NewPermission = $state({
    host_id: 0,
    service_name: "",
    can_view: false,
    can_act: false
  });

  let error = $state("");

  $effect(() => {
    api.permission.get(userId, permId).then((perm: Permission) => {
      formData = pick(perm, Object.keys(formData)) as NewPermission;
    });
  });

  const relatedSelects = {
    host_id: {
      listApi: api.host.list,
      idFunc: (h: Host) => h.id,
      displayFunc: (h: Host) => h.name
    }
  };
</script>

<EntityForm {formData} {error} {onSubmit} {relatedSelects} actionBtnText="Update" />
