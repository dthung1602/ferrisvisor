<script lang="ts">
  import EntityForm from "$lib/components/EntityForm.svelte";
  import type { Group } from "$lib/api/group";
  import { group } from "$lib/api";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { page } from "$app/state";
  import { omit } from "lodash";

  type NewGroup = Omit<Group, "id" | "created_at" | "updated_at">;

  const groupId = parseInt(page.params.id || "0");

  let error = $state("");

  async function onSubmit(formData: NewGroup) {
    try {
      await group.update(groupId, formData);
      await goto(resolve("/admin/groups"));
    } catch (e) {
      console.error(e);
      error = e + "";
    }
  }

  let formData: NewGroup = $state({
    name: "",
    description: ""
  });

  $effect(() => {
    group.get(groupId).then((groupData) => {
      formData = omit(groupData, ["id", "created_at", "updated_at"]);
    });
  });
</script>

<EntityForm {formData} {error} {onSubmit} relatedSelects={{}} actionBtnText="Update" />
