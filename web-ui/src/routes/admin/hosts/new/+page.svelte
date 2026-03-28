<script lang="ts">
  import EntityForm from "$lib/components/EntityForm.svelte";
  import type { Host } from "$lib/api/host";
  import type { Group } from "$lib/api/group";
  import { group, host } from "$lib/api";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";

  type NewHost = Omit<Host, "id" | "created_at" | "updated_at">;

  let error  = $state("");

  async function onSubmit(formData: NewHost) {
    try {
      await host.create(formData);
      await goto(resolve("/admin/hosts"));
    } catch (e) {
      console.error(e);
      error = e + ""
    }
  }

  const formData = {
    group_id: 0,
    name: "",
    hostname: "",
    port: 0,
    username: "",
    password: "",
  } as NewHost;

  const relatedSelects = {
    "group_id": {
      listApi: group.list,
      idFunc: (g: Group) => g.id,
      displayFunc: (g: Group) => g.name
    }
  };
</script>

<EntityForm
  formData={formData}
  relatedSelects={relatedSelects}
  error={error}
  onSubmit={onSubmit}
/>
