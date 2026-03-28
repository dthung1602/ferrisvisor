<script lang="ts">
  import EntityForm from "$lib/components/EntityForm.svelte";
  import type { Group } from "$lib/api/group";
  import { group } from "$lib/api";
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";

  type NewGroup = Omit<Group, "id" | "created_at" | "updated_at">;

  let error  = $state("");

  async function onSubmit(formData: NewGroup) {
    try {
      await group.create(formData);
      await goto(resolve("/admin/groups"));
    } catch (e) {
      console.error(e);
      error = e + ""
    }
  }

  const formData = {
    name: "",
    description: ""
  } as NewGroup;
</script>

<EntityForm
  formData={formData}
  error={error}
  onSubmit={onSubmit}
/>
