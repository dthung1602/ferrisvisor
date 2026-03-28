<script lang="ts">
  import EntityForm from "$lib/components/EntityForm.svelte";
  import type { Host } from "$lib/api/host";
  import type { Group } from "$lib/api/group";
  import { host, group } from "$lib/api";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { omit } from "lodash";

  type NewHost = Omit<Host, "id" | "created_at" | "updated_at">;

  const hostId = parseInt(page.params.id || "0");

  let error = $state("");

  async function onSubmit(formData: NewHost) {
    try {
      await host.update(hostId, formData);
      await goto(resolve("/admin/hosts"));
    } catch (e) {
      console.error(e);
      error = e + "";
    }
  }

  let formData: NewHost = $state({
    group_id: 0,
    name: "",
    hostname: "",
    port: 0,
    username: "",
    password: "",
  });

  const relatedSelects = {
    "group_id": {
      listApi: group.list,
      idFunc: (g: Group) => g.id,
      displayFunc: (g: Group) => g.name
    }
  };

  $effect(() => {
    host.get(hostId).then((hostData) => {
      formData = omit(hostData, ["id", "created_at", "updated_at"]);
    })
  })
</script>

<EntityForm {formData} {error} {onSubmit} {relatedSelects} actionBtnText="Update" />
